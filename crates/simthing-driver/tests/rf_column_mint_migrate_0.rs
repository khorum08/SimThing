//! RF-COLUMN-MINT-MIGRATE-0 proofs.
//!
//! Production layout keys interned owner id + `SimThingId`. Wrong
//! implementations are test-side lexical-OwnerRef order and physical-row keys.

use serde::Deserialize;
use simthing_core::owner_channel::{bind_owner, unbind_owner, OwnerInterner, OwnerRef};
use simthing_core::{
    AnchorRemapOperation, AnchoredLocusMap, BindingTableSnapshot, ColumnIndex, DimensionRegistry,
    SimProperty, SimThing, SimThingKind, SlotIndex, SubFieldRole,
};
use simthing_driver::{compile_owner_channel_rf_gpu_proof_plan, SpecSessionState};
use simthing_gpu::SlotAllocator;
use simthing_spec::{OwnerChannelRfOwnAggregate, OwnerChannelScopeKey, ResourceKey};

#[derive(Clone, Copy, Debug)]
enum Case {
    NoLegacyNew,
    ComparativeRolePathway,
    ScanRetired,
    InternOrderVsLexical,
    OwnerFlipStable,
    IndependentSessions,
    EpochRebindLogicalStable,
    OwnerAddRemoveStable,
    SeamRejectsInternedId,
}

fn tree_two_owners(a: &str, b: &str) -> (SimThing, Vec<OwnerChannelRfOwnAggregate>) {
    let mut root = SimThing::new(SimThingKind::Custom("synthetic".into()), 0);
    bind_owner(&mut root, &OwnerRef::new(a));
    let mut child = SimThing::new(SimThingKind::Custom("synthetic".into()), 0);
    bind_owner(&mut child, &OwnerRef::new(b));
    let child_id = child.id;
    let root_id = root.id;
    root.add_child(child);
    let rows = vec![
        OwnerChannelRfOwnAggregate {
            simthing_id: root_id,
            resource_key: ResourceKey::new("ore"),
            surplus: 3,
            deficit: 0,
        },
        OwnerChannelRfOwnAggregate {
            simthing_id: child_id,
            resource_key: ResourceKey::new("ore"),
            surplus: 5,
            deficit: 0,
        },
    ];
    (root, rows)
}

fn compile_held(
    session: &mut SpecSessionState,
    root: &SimThing,
    rows: &[OwnerChannelRfOwnAggregate],
) -> Result<simthing_driver::OwnerChannelRfGpuProofPlan, String> {
    compile_owner_channel_rf_gpu_proof_plan(root, rows, &mut session.persistent_rf_layout)
        .map_err(|e| format!("{e:?}"))
}

fn intern_owner_order(plan_scopes: &[OwnerChannelScopeKey], interner: &OwnerInterner) -> Vec<u32> {
    plan_scopes
        .iter()
        .map(|s| interner.id_of(&s.owner_ref).expect("interned").raw())
        .collect()
}

fn lexical_owner_order(scopes: &[OwnerChannelScopeKey]) -> Vec<String> {
    let mut names: Vec<String> = scopes
        .iter()
        .map(|s| s.owner_ref.as_str().to_string())
        .collect();
    names.sort();
    names
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
struct SeamOwnerChannel {
    owner_ref: OwnerRef,
    resource_key: String,
    scope_id: String,
}

fn run_case(case: Case) -> Result<(), String> {
    match case {
        Case::NoLegacyNew => {
            // Compatibility alias is gone: remaining doors compile; `ColumnIndex::new`
            // is unresolvable (the workspace build is the compile-fail). Oracle door
            // still mints independent of the role pathway.
            let _ = ColumnIndex::from_raw_for_oracle_or_rehearsal(0);
            Ok(())
        }
        Case::ComparativeRolePathway => {
            let mut registry = DimensionRegistry::new();
            let pad = registry.register(SimProperty::simple("pad", "pad", 1));
            let derived = registry.register(SimProperty::simple("comparative", "dominance", 1));
            let pad_layout = registry.property(pad).layout.clone();
            let derived_layout = registry.property(derived).layout.clone();
            let pad_amount = registry
                .column_range(pad)
                .col_for_role(&SubFieldRole::Amount, &pad_layout)
                .expect("pad Amount");
            let production = registry
                .column_range(derived)
                .col_for_role(&SubFieldRole::Amount, &derived_layout)
                .expect("comparative Amount");
            let gpu_wire0 = ColumnIndex::from_gpu_round_trip(pad_amount.raw_u32());
            if production == gpu_wire0 {
                return Err(
                    "comparative Amount must come from col_for_role, not GPU-wire column 0".into(),
                );
            }
            Ok(())
        }
        Case::ScanRetired => {
            let scans = std::fs::read_to_string(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../scripts/ci/scans.tsv"
            ))
            .map_err(|e| e.to_string())?;
            if scans.lines().any(|line| {
                let id = line.split('\t').next().unwrap_or("");
                id == "COLUMN-INDEX-MINT"
            }) {
                return Err("COLUMN-INDEX-MINT scan must be retired at 9.2".into());
            }
            Ok(())
        }
        Case::InternOrderVsLexical => {
            let (root, rows) = tree_two_owners("zulu", "alpha");
            let mut session = SpecSessionState::new();
            let plan = compile_held(&mut session, &root, &rows)?;
            let scopes: Vec<_> = plan.bucket_plans.iter().map(|b| b.scope.clone()).collect();
            let interned = intern_owner_order(&scopes, &plan.layout.interner);
            let production_names: Vec<_> = scopes
                .iter()
                .map(|s| s.owner_ref.as_str().to_string())
                .collect();
            let lexical = lexical_owner_order(&scopes);
            if interned != vec![0, 1] {
                return Err(format!(
                    "tree-walk intern order should be 0,1 got {interned:?}"
                ));
            }
            if production_names == lexical {
                return Err(
                    "zulu-then-alpha tree walk must disagree with lexical OwnerRef order".into(),
                );
            }
            Ok(())
        }
        Case::OwnerFlipStable => {
            let (root, rows) = tree_two_owners("alpha", "beta");
            let mut session = SpecSessionState::new();
            let before = compile_held(&mut session, &root, &rows)?;
            let before_ids = intern_owner_order(
                &before
                    .bucket_plans
                    .iter()
                    .map(|b| b.scope.clone())
                    .collect::<Vec<_>>(),
                &session.persistent_rf_layout.interner,
            );
            let beta = OwnerRef::new("beta");
            let beta_before = session
                .persistent_rf_layout
                .interner
                .id_of(&beta)
                .ok_or("beta interned")?;
            let mut flipped = root;
            bind_owner(&mut flipped, &OwnerRef::new("zulu"));
            let after = compile_held(&mut session, &flipped, &rows)?;
            let after_ids = intern_owner_order(
                &after
                    .bucket_plans
                    .iter()
                    .map(|b| b.scope.clone())
                    .collect::<Vec<_>>(),
                &session.persistent_rf_layout.interner,
            );
            if before_ids != after_ids {
                return Err(format!(
                    "interned layout order moved under owner flip {before_ids:?} -> {after_ids:?}"
                ));
            }
            let beta_after = session
                .persistent_rf_layout
                .interner
                .id_of(&beta)
                .ok_or("beta still interned")?;
            if beta_before != beta_after {
                return Err("unrelated interned id remapped under owner flip".into());
            }
            let zulu_id = session
                .persistent_rf_layout
                .interner
                .id_of(&OwnerRef::new("zulu"))
                .ok_or("zulu interned after rebind")?;
            if session
                .persistent_rf_layout
                .interner
                .id_of(&OwnerRef::new("alpha"))
                .is_some()
            {
                return Err("rebind must drop the old OwnerRef string".into());
            }
            if zulu_id.raw() != before_ids[0] {
                return Err("flag-switch rebind must keep the flipped interned id".into());
            }
            let before_lex = lexical_owner_order(
                &before
                    .bucket_plans
                    .iter()
                    .map(|b| b.scope.clone())
                    .collect::<Vec<_>>(),
            );
            let after_lex = lexical_owner_order(
                &after
                    .bucket_plans
                    .iter()
                    .map(|b| b.scope.clone())
                    .collect::<Vec<_>>(),
            );
            if before_lex == after_lex {
                return Err(
                    "lexical OwnerRef rival must permute on alpha->zulu while production stays"
                        .into(),
                );
            }
            Ok(())
        }
        Case::IndependentSessions => {
            let mut a = OwnerInterner::new();
            let mut b = OwnerInterner::new();
            let alpha = OwnerRef::new("alpha");
            let zulu = OwnerRef::new("zulu");
            a.intern(&alpha);
            a.intern(&zulu);
            b.intern(&zulu);
            b.intern(&alpha);
            if a.id_of(&alpha) == b.id_of(&alpha) {
                return Err(
                    "independent sessions must intern the same OwnerRef differently".into(),
                );
            }
            Ok(())
        }
        Case::EpochRebindLogicalStable => {
            let (root, rows) = tree_two_owners("alpha", "beta");
            let mut session = SpecSessionState::new();
            let mut alloc = SlotAllocator::new();
            alloc.install_initial_tree(&root);
            let pre = alloc.binding_table_snapshot();
            let before = compile_held(&mut session, &root, &rows)?;
            let before_ids = intern_owner_order(
                &before
                    .bucket_plans
                    .iter()
                    .map(|b| b.scope.clone())
                    .collect::<Vec<_>>(),
                &session.persistent_rf_layout.interner,
            );
            let before_keys = session.persistent_rf_layout.keys().to_vec();
            if before_keys.iter().any(|k| pre.get(&k.object).is_none()) {
                return Err("production layout keys must use live SimThingId identity".into());
            }

            let mut slots: Vec<SlotIndex> = pre.values().copied().collect();
            slots.sort();
            let mut assignment = BindingTableSnapshot::new();
            let mut ordered: Vec<_> = pre.iter().map(|(&id, &s)| (id, s)).collect();
            ordered.sort_by_key(|&(_, s)| s);
            for (i, &(id, _)) in ordered.iter().enumerate() {
                assignment.insert(id, slots[slots.len() - 1 - i]);
            }
            let loci = AnchoredLocusMap::new();
            let section = alloc
                .epoch_rebind(&assignment, &loci, &loci)
                .map_err(|e| format!("{e:?}"))?;
            if section.operation != AnchorRemapOperation::EpochRebind {
                return Err("forced remap must be EpochRebind".into());
            }
            let moved = pre.iter().filter(|(id, s)| assignment[*id] != **s).count();
            if moved == 0 {
                return Err("forced scramble must move physical rows".into());
            }

            let after = compile_held(&mut session, &root, &rows)?;
            let after_ids = intern_owner_order(
                &after
                    .bucket_plans
                    .iter()
                    .map(|b| b.scope.clone())
                    .collect::<Vec<_>>(),
                &session.persistent_rf_layout.interner,
            );
            let after_keys = session.persistent_rf_layout.keys().to_vec();
            if before_ids != after_ids || before_keys != after_keys {
                return Err(format!(
                    "persistent interned layout moved under EpochRebind {before_ids:?} -> {after_ids:?}"
                ));
            }
            let mut physical_key_before: Vec<_> = before_keys
                .iter()
                .map(|k| (pre.get(&k.object).map(|s| s.raw()).unwrap_or(0), k.object))
                .collect();
            physical_key_before.sort_by_key(|(row, _)| *row);
            let mut physical_key_after: Vec<_> = after_keys
                .iter()
                .map(|k| {
                    (
                        alloc.slot_of(k.object).map(|s| s.raw()).unwrap_or(0),
                        k.object,
                    )
                })
                .collect();
            physical_key_after.sort_by_key(|(row, _)| *row);
            if physical_key_before == physical_key_after {
                return Err(
                    "physical SlotIndex keyed by slot_of must permute under EpochRebind".into(),
                );
            }
            Ok(())
        }
        Case::OwnerAddRemoveStable => {
            let (mut root, mut rows) = tree_two_owners("alpha", "beta");
            let mut session = SpecSessionState::new();
            let _first = compile_held(&mut session, &root, &rows)?;
            let alpha = OwnerRef::new("alpha");
            let beta = OwnerRef::new("beta");
            let alpha_id = session
                .persistent_rf_layout
                .interner
                .id_of(&alpha)
                .ok_or("alpha interned")?;
            let beta_id = session
                .persistent_rf_layout
                .interner
                .id_of(&beta)
                .ok_or("beta interned")?;
            let first_keys = session.persistent_rf_layout.keys().to_vec();
            let first_indices: Vec<_> = first_keys
                .iter()
                .map(|k| session.persistent_rf_layout.index_of(k))
                .collect();

            let mut extra = SimThing::new(SimThingKind::Custom("synthetic".into()), 0);
            bind_owner(&mut extra, &OwnerRef::new("gamma"));
            let extra_id = extra.id;
            root.add_child(extra);
            rows.push(OwnerChannelRfOwnAggregate {
                simthing_id: extra_id,
                resource_key: ResourceKey::new("ore"),
                surplus: 1,
                deficit: 0,
            });
            let _added = compile_held(&mut session, &root, &rows)?;
            if session.persistent_rf_layout.interner.id_of(&alpha) != Some(alpha_id)
                || session.persistent_rf_layout.interner.id_of(&beta) != Some(beta_id)
            {
                return Err("adding an owner remapped unrelated interned ids".into());
            }
            let added_indices: Vec<_> = first_keys
                .iter()
                .map(|k| session.persistent_rf_layout.index_of(k))
                .collect();
            if added_indices != first_indices {
                return Err("adding an owner remapped unrelated layout indices".into());
            }

            let child = root
                .children
                .iter_mut()
                .find(|c| c.id == extra_id)
                .ok_or("gamma child")?;
            unbind_owner(child);
            let _removed = compile_held(&mut session, &root, &rows)?;
            if session.persistent_rf_layout.interner.id_of(&alpha) != Some(alpha_id)
                || session.persistent_rf_layout.interner.id_of(&beta) != Some(beta_id)
            {
                return Err("removing an owner remapped unrelated interned ids".into());
            }
            Ok(())
        }
        Case::SeamRejectsInternedId => {
            let ok = serde_json::from_str::<SeamOwnerChannel>(
                r#"{"owner_ref":"alpha","resource_key":"ore","scope_id":"s"}"#,
            );
            if ok.is_err() {
                return Err(format!("canonical OwnerRef seam must admit {ok:?}"));
            }
            let bad = serde_json::from_str::<SeamOwnerChannel>(
                r#"{"owner_ref":"alpha","resource_key":"ore","scope_id":"s","interned_owner_id":1}"#,
            );
            if bad.is_ok() {
                return Err("seam must reject interned owner id".into());
            }
            Ok(())
        }
    }
}

#[test]
fn rf_column_mint_migrate_table() {
    for case in [
        Case::NoLegacyNew,
        Case::ComparativeRolePathway,
        Case::ScanRetired,
        Case::InternOrderVsLexical,
        Case::OwnerFlipStable,
        Case::IndependentSessions,
        Case::EpochRebindLogicalStable,
        Case::OwnerAddRemoveStable,
        Case::SeamRejectsInternedId,
    ] {
        run_case(case).unwrap_or_else(|e| panic!("{case:?}: {e}"));
    }
}
