//! Deterministic resident economic-resolution plan substrate.
//!
//! This module owns semantic dictionaries and dense ordinal layout only. It
//! intentionally contains no scoring, equality-band, apportionment, grant,
//! dispatch, or structural-consequence implementation.

use serde::{Deserialize, Serialize};
use simthing_core::{
    GenerationStamp, RealmQualified, SeamFact, SimThingId, TreeExecutionContext,
    TreeExecutionContextError, TreeRealmId,
};
use thiserror::Error;

const CANONICAL_VERSION: u32 = 1;
const CANONICAL_DOMAIN: &[u8; 8] = b"STRCP140";

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ResidentOwnerId(RealmQualified<SimThingId>);

impl ResidentOwnerId {
    pub const fn new(identity: RealmQualified<SimThingId>) -> Self {
        Self(identity)
    }

    pub const fn identity(self) -> RealmQualified<SimThingId> {
        self.0
    }

    fn append_canonical(self, bytes: &mut Vec<u8>) {
        bytes.extend_from_slice(&self.0.realm().canonical_bytes());
        bytes.extend_from_slice(&self.0.local().raw().to_le_bytes());
    }
}

macro_rules! semantic_id {
    ($name:ident) => {
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
        )]
        #[serde(transparent)]
        pub struct $name(u64);

        impl $name {
            pub const fn new(value: u64) -> Self {
                Self(value)
            }

            pub const fn get(self) -> u64 {
                self.0
            }
        }
    };
}

semantic_id!(ResidentResourceId);
semantic_id!(ResidentScopeId);
semantic_id!(ResidentDrawId);

macro_rules! ordinal_id {
    ($name:ident) => {
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
        )]
        #[serde(transparent)]
        pub struct $name(u32);

        impl $name {
            pub const fn get(self) -> u32 {
                self.0
            }
        }
    };
}

ordinal_id!(ResidentOwnerOrdinal);
ordinal_id!(ResidentResourceOrdinal);
ordinal_id!(ResidentScopeOrdinal);
ordinal_id!(ResidentDrawOrdinal);

/// One admitted semantic composition row before dense ordinal assignment.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResidentClearingAdmission {
    pub owner: ResidentOwnerId,
    pub resource: ResidentResourceId,
    pub scope: ResidentScopeId,
    pub draw: ResidentDrawId,
}

/// One canonical dense resident row. The four ordinal axes are deliberately
/// typed so transposition is uncompilable.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResidentClearingRow {
    owner: ResidentOwnerOrdinal,
    resource: ResidentResourceOrdinal,
    scope: ResidentScopeOrdinal,
    draw: ResidentDrawOrdinal,
}

impl ResidentClearingRow {
    pub const fn owner(self) -> ResidentOwnerOrdinal {
        self.owner
    }

    pub const fn resource(self) -> ResidentResourceOrdinal {
        self.resource
    }

    pub const fn scope(self) -> ResidentScopeOrdinal {
        self.scope
    }

    pub const fn draw(self) -> ResidentDrawOrdinal {
        self.draw
    }
}

/// Checked half-open range in one dense ordinal domain.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DenseOrdinalRange {
    start: u32,
    len: u32,
}

impl DenseOrdinalRange {
    pub fn try_new(start: u32, len: u32) -> Result<Self, ResidentClearingPlanError> {
        start
            .checked_add(len)
            .ok_or(ResidentClearingPlanError::OrdinalRangeOverflow { start, len })?;
        Ok(Self { start, len })
    }

    pub const fn start(self) -> u32 {
        self.start
    }

    pub const fn len(self) -> u32 {
        self.len
    }

    pub const fn is_empty(self) -> bool {
        self.len == 0
    }

    pub fn end(self) -> u32 {
        self.start + self.len
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResidentClearingRanges {
    pub owners: DenseOrdinalRange,
    pub resources: DenseOrdinalRange,
    pub scopes: DenseOrdinalRange,
    pub draws: DenseOrdinalRange,
    pub rows: DenseOrdinalRange,
}

/// Admission budgets checked before any GPU allocation can be requested.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResidentClearingBudgets {
    max_owners: u32,
    max_resources: u32,
    max_scopes: u32,
    max_draws: u32,
    max_rows: u32,
    max_semantic_plan_bytes: u64,
    max_resident_bytes: u64,
    max_scratch_bytes: u64,
    scratch_bytes_per_row: u32,
}

impl ResidentClearingBudgets {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        max_owners: u32,
        max_resources: u32,
        max_scopes: u32,
        max_draws: u32,
        max_rows: u32,
        max_semantic_plan_bytes: u64,
        max_resident_bytes: u64,
        max_scratch_bytes: u64,
        scratch_bytes_per_row: u32,
    ) -> Result<Self, ResidentClearingPlanError> {
        if max_owners == 0
            || max_resources == 0
            || max_scopes == 0
            || max_draws == 0
            || max_rows == 0
            || max_semantic_plan_bytes == 0
            || max_resident_bytes == 0
            || max_scratch_bytes == 0
            || scratch_bytes_per_row == 0
        {
            return Err(ResidentClearingPlanError::ZeroBudget);
        }
        let minimum_scratch = u64::from(max_rows)
            .checked_mul(u64::from(scratch_bytes_per_row))
            .ok_or(ResidentClearingPlanError::BudgetArithmeticOverflow {
                field: "max_rows*scratch_bytes_per_row",
            })?;
        if minimum_scratch > max_scratch_bytes {
            return Err(ResidentClearingPlanError::ScratchBudgetInconsistent {
                required: minimum_scratch,
                admitted: max_scratch_bytes,
            });
        }
        Ok(Self {
            max_owners,
            max_resources,
            max_scopes,
            max_draws,
            max_rows,
            max_semantic_plan_bytes,
            max_resident_bytes,
            max_scratch_bytes,
            scratch_bytes_per_row,
        })
    }

    pub const fn max_owners(self) -> u32 {
        self.max_owners
    }
    pub const fn max_resources(self) -> u32 {
        self.max_resources
    }
    pub const fn max_scopes(self) -> u32 {
        self.max_scopes
    }
    pub const fn max_draws(self) -> u32 {
        self.max_draws
    }
    pub const fn max_rows(self) -> u32 {
        self.max_rows
    }
    pub const fn max_semantic_plan_bytes(self) -> u64 {
        self.max_semantic_plan_bytes
    }
    pub const fn max_resident_bytes(self) -> u64 {
        self.max_resident_bytes
    }
    pub const fn max_scratch_bytes(self) -> u64 {
        self.max_scratch_bytes
    }
    pub const fn scratch_bytes_per_row(self) -> u32 {
        self.scratch_bytes_per_row
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticPlanDigest {
    low: u64,
    high: u64,
}

impl SemanticPlanDigest {
    pub const fn low(self) -> u64 {
        self.low
    }

    pub const fn high(self) -> u64 {
        self.high
    }

    pub fn to_hex(self) -> String {
        format!("{:016x}{:016x}", self.high, self.low)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResidentPlanContext {
    realm: TreeRealmId,
    root: SimThingId,
    generation: GenerationStamp,
}

impl ResidentPlanContext {
    pub const fn realm(self) -> TreeRealmId {
        self.realm
    }
    pub const fn root(self) -> SimThingId {
        self.root
    }
    pub const fn generation(self) -> GenerationStamp {
        self.generation
    }
}

impl From<TreeExecutionContext> for ResidentPlanContext {
    fn from(context: TreeExecutionContext) -> Self {
        Self {
            realm: context.realm(),
            root: context.root(),
            generation: context.generation(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResidentClearingDictionaries {
    owners: Vec<ResidentOwnerId>,
    resources: Vec<ResidentResourceId>,
    scopes: Vec<ResidentScopeId>,
    draws: Vec<ResidentDrawId>,
}

impl ResidentClearingDictionaries {
    pub fn owners(&self) -> &[ResidentOwnerId] {
        &self.owners
    }
    pub fn resources(&self) -> &[ResidentResourceId] {
        &self.resources
    }
    pub fn scopes(&self) -> &[ResidentScopeId] {
        &self.scopes
    }
    pub fn draws(&self) -> &[ResidentDrawId] {
        &self.draws
    }
}

/// Immutable semantic plan whose bytes depend only on admitted semantic
/// identity and canonical total order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResidentClearingPlan {
    context: ResidentPlanContext,
    dictionaries: ResidentClearingDictionaries,
    ranges: ResidentClearingRanges,
    rows: Vec<ResidentClearingRow>,
    budgets: ResidentClearingBudgets,
    digest: SemanticPlanDigest,
}

impl ResidentClearingPlan {
    pub fn build(
        context: TreeExecutionContext,
        admissions: impl IntoIterator<Item = ResidentClearingAdmission>,
        budgets: ResidentClearingBudgets,
    ) -> Result<Self, ResidentClearingPlanError> {
        let admissions: Vec<_> = admissions.into_iter().collect();
        if admissions.is_empty() {
            return Err(ResidentClearingPlanError::EmptyAdmissions);
        }

        let owners = sorted_unique(admissions.iter().map(|row| row.owner));
        let resources = sorted_unique(admissions.iter().map(|row| row.resource));
        let scopes = sorted_unique(admissions.iter().map(|row| row.scope));
        let draws = sorted_unique(admissions.iter().map(|row| row.draw));

        let owner_count = checked_count("owners", owners.len(), budgets.max_owners())?;
        let resource_count = checked_count("resources", resources.len(), budgets.max_resources())?;
        let scope_count = checked_count("scopes", scopes.len(), budgets.max_scopes())?;
        let draw_count = checked_count("draws", draws.len(), budgets.max_draws())?;
        let row_count = checked_count("rows", admissions.len(), budgets.max_rows())?;

        let mut rows = Vec::with_capacity(admissions.len());
        for admission in admissions {
            rows.push(ResidentClearingRow {
                owner: ResidentOwnerOrdinal(binary_ordinal(&owners, admission.owner)?),
                resource: ResidentResourceOrdinal(binary_ordinal(&resources, admission.resource)?),
                scope: ResidentScopeOrdinal(binary_ordinal(&scopes, admission.scope)?),
                draw: ResidentDrawOrdinal(binary_ordinal(&draws, admission.draw)?),
            });
        }
        rows.sort_unstable();
        if rows.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err(ResidentClearingPlanError::DuplicateAdmission);
        }

        let dictionaries = ResidentClearingDictionaries {
            owners,
            resources,
            scopes,
            draws,
        };
        let ranges = ResidentClearingRanges {
            owners: DenseOrdinalRange::try_new(0, owner_count)?,
            resources: DenseOrdinalRange::try_new(0, resource_count)?,
            scopes: DenseOrdinalRange::try_new(0, scope_count)?,
            draws: DenseOrdinalRange::try_new(0, draw_count)?,
            rows: DenseOrdinalRange::try_new(0, row_count)?,
        };
        let mut plan = Self {
            context: context.into(),
            dictionaries,
            ranges,
            rows,
            budgets,
            digest: SemanticPlanDigest { low: 0, high: 0 },
        };
        let semantic_bytes = plan.canonical_byte_len()?;
        if semantic_bytes > budgets.max_semantic_plan_bytes() {
            return Err(ResidentClearingPlanError::SemanticPlanBudgetExceeded {
                required: semantic_bytes,
                admitted: budgets.max_semantic_plan_bytes(),
            });
        }
        usize::try_from(semantic_bytes).map_err(|_| {
            ResidentClearingPlanError::BudgetArithmeticOverflow {
                field: "canonical_plan_host_bytes",
            }
        })?;
        let canonical = plan.canonical_bytes_without_digest();
        plan.digest = digest_bytes(&canonical);
        Ok(plan)
    }

    pub const fn context(&self) -> ResidentPlanContext {
        self.context
    }

    pub const fn ranges(&self) -> ResidentClearingRanges {
        self.ranges
    }

    pub const fn budgets(&self) -> ResidentClearingBudgets {
        self.budgets
    }

    pub const fn digest(&self) -> SemanticPlanDigest {
        self.digest
    }

    pub fn dictionaries(&self) -> &ResidentClearingDictionaries {
        &self.dictionaries
    }

    pub fn rows(&self) -> &[ResidentClearingRow] {
        &self.rows
    }

    pub fn canonical_bytes(&self) -> Vec<u8> {
        self.canonical_bytes_without_digest()
    }

    pub fn bind_context(
        &self,
        context: TreeExecutionContext,
    ) -> Result<ResidentClearingPlanBinding, ResidentClearingPlanError> {
        let observed = ResidentPlanContext::from(context);
        if observed != self.context {
            return Err(ResidentClearingPlanError::ContextMismatch {
                expected: self.context,
                observed,
            });
        }
        Ok(ResidentClearingPlanBinding {
            realm: context.realm(),
            incarnation: context.incarnation(),
            generation: context.generation(),
            digest: self.digest,
        })
    }

    /// Remap a canonical foreign identity into this plan's local owner
    /// ordinal. No API accepts the fact's source ordinal as destination input.
    pub fn remap_seam_owner(
        &self,
        source_context: TreeExecutionContext,
        fact: &SeamFact<SimThingId>,
    ) -> Result<ResidentOwnerOrdinal, ResidentClearingPlanError> {
        source_context.admit_seam_fact(fact)?;
        self.owner_ordinal(ResidentOwnerId::new(*fact.subject()))
    }

    pub fn owner_ordinal(
        &self,
        owner: ResidentOwnerId,
    ) -> Result<ResidentOwnerOrdinal, ResidentClearingPlanError> {
        let index = self
            .dictionaries
            .owners
            .binary_search(&owner)
            .map_err(|_| ResidentClearingPlanError::UnknownOwner(owner))?;
        let ordinal = u32::try_from(index).map_err(|_| {
            ResidentClearingPlanError::BudgetArithmeticOverflow {
                field: "owner_ordinal",
            }
        })?;
        Ok(ResidentOwnerOrdinal(ordinal))
    }

    fn canonical_bytes_without_digest(&self) -> Vec<u8> {
        let capacity = usize::try_from(
            self.canonical_byte_len()
                .expect("a constructed plan has a checked canonical byte length"),
        )
        .expect("a constructed plan has a host-representable canonical byte length");
        let mut bytes = Vec::with_capacity(capacity);
        bytes.extend_from_slice(CANONICAL_DOMAIN);
        bytes.extend_from_slice(&CANONICAL_VERSION.to_le_bytes());
        bytes.extend_from_slice(&self.context.realm.canonical_bytes());
        bytes.extend_from_slice(&self.context.root.raw().to_le_bytes());
        bytes.extend_from_slice(&self.context.generation.get().to_le_bytes());
        append_budgets(&mut bytes, self.budgets);
        append_count(&mut bytes, self.ranges.owners.len());
        for owner in &self.dictionaries.owners {
            owner.append_canonical(&mut bytes);
        }
        append_count(&mut bytes, self.ranges.resources.len());
        for id in &self.dictionaries.resources {
            bytes.extend_from_slice(&id.get().to_le_bytes());
        }
        append_count(&mut bytes, self.ranges.scopes.len());
        for id in &self.dictionaries.scopes {
            bytes.extend_from_slice(&id.get().to_le_bytes());
        }
        append_count(&mut bytes, self.ranges.draws.len());
        for id in &self.dictionaries.draws {
            bytes.extend_from_slice(&id.get().to_le_bytes());
        }
        append_count(&mut bytes, self.ranges.rows.len());
        for row in &self.rows {
            bytes.extend_from_slice(&row.owner.get().to_le_bytes());
            bytes.extend_from_slice(&row.resource.get().to_le_bytes());
            bytes.extend_from_slice(&row.scope.get().to_le_bytes());
            bytes.extend_from_slice(&row.draw.get().to_le_bytes());
        }
        bytes
    }

    fn canonical_byte_len(&self) -> Result<u64, ResidentClearingPlanError> {
        // domain/version/context + budgets + five encoded dictionary/row counts
        let mut bytes = 8_u64
            .checked_add(4)
            .and_then(|value| value.checked_add(16))
            .and_then(|value| value.checked_add(4))
            .and_then(|value| value.checked_add(4))
            .and_then(|value| value.checked_add(48))
            .and_then(|value| value.checked_add(5 * 8))
            .ok_or(ResidentClearingPlanError::BudgetArithmeticOverflow {
                field: "canonical_fixed_bytes",
            })?;
        bytes = checked_axis_bytes(bytes, self.ranges.owners.len(), 20, "owner_bytes")?;
        bytes = checked_axis_bytes(bytes, self.ranges.resources.len(), 8, "resource_bytes")?;
        bytes = checked_axis_bytes(bytes, self.ranges.scopes.len(), 8, "scope_bytes")?;
        bytes = checked_axis_bytes(bytes, self.ranges.draws.len(), 8, "draw_bytes")?;
        checked_axis_bytes(bytes, self.ranges.rows.len(), 16, "row_bytes")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResidentClearingPlanBinding {
    realm: TreeRealmId,
    incarnation: simthing_core::ExecutionIncarnation,
    generation: GenerationStamp,
    digest: SemanticPlanDigest,
}

impl ResidentClearingPlanBinding {
    pub const fn realm(self) -> TreeRealmId {
        self.realm
    }
    pub const fn incarnation(self) -> simthing_core::ExecutionIncarnation {
        self.incarnation
    }
    pub const fn generation(self) -> GenerationStamp {
        self.generation
    }
    pub const fn digest(self) -> SemanticPlanDigest {
        self.digest
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum ResidentClearingPlanError {
    #[error("resident clearing admissions must not be empty")]
    EmptyAdmissions,
    #[error("resident clearing admission rows must be unique")]
    DuplicateAdmission,
    #[error("resident clearing budgets must be non-zero")]
    ZeroBudget,
    #[error("budget arithmetic overflow in {field}")]
    BudgetArithmeticOverflow { field: &'static str },
    #[error("scratch budget is inconsistent: required {required}, admitted {admitted}")]
    ScratchBudgetInconsistent { required: u64, admitted: u64 },
    #[error("{axis} count {observed} exceeds admitted maximum {admitted}")]
    CountBudgetExceeded {
        axis: &'static str,
        observed: u64,
        admitted: u32,
    },
    #[error("dense ordinal range overflow: start {start}, len {len}")]
    OrdinalRangeOverflow { start: u32, len: u32 },
    #[error("canonical semantic plan requires {required} bytes, admitted {admitted}")]
    SemanticPlanBudgetExceeded { required: u64, admitted: u64 },
    #[error("resident plan context mismatch: expected {expected:?}, observed {observed:?}")]
    ContextMismatch {
        expected: ResidentPlanContext,
        observed: ResidentPlanContext,
    },
    #[error("canonical dictionary lookup failed during plan construction")]
    DictionaryConstruction,
    #[error("destination plan has no ordinal for canonical owner {0:?}")]
    UnknownOwner(ResidentOwnerId),
    #[error(transparent)]
    SeamAdmission(#[from] TreeExecutionContextError),
}

fn sorted_unique<T: Ord>(values: impl IntoIterator<Item = T>) -> Vec<T> {
    let mut values: Vec<_> = values.into_iter().collect();
    values.sort_unstable();
    values.dedup();
    values
}

fn checked_count(
    axis: &'static str,
    observed: usize,
    admitted: u32,
) -> Result<u32, ResidentClearingPlanError> {
    let observed_u64 = u64::try_from(observed)
        .map_err(|_| ResidentClearingPlanError::BudgetArithmeticOverflow { field: axis })?;
    if observed_u64 > u64::from(admitted) || observed_u64 > u64::from(u32::MAX) {
        return Err(ResidentClearingPlanError::CountBudgetExceeded {
            axis,
            observed: observed_u64,
            admitted,
        });
    }
    u32::try_from(observed_u64)
        .map_err(|_| ResidentClearingPlanError::BudgetArithmeticOverflow { field: axis })
}

fn checked_axis_bytes(
    accumulated: u64,
    count: u32,
    stride: u64,
    field: &'static str,
) -> Result<u64, ResidentClearingPlanError> {
    u64::from(count)
        .checked_mul(stride)
        .and_then(|axis_bytes| accumulated.checked_add(axis_bytes))
        .ok_or(ResidentClearingPlanError::BudgetArithmeticOverflow { field })
}

fn binary_ordinal<T: Ord>(dictionary: &[T], value: T) -> Result<u32, ResidentClearingPlanError> {
    let index = dictionary
        .binary_search(&value)
        .map_err(|_| ResidentClearingPlanError::DictionaryConstruction)?;
    u32::try_from(index).map_err(|_| ResidentClearingPlanError::BudgetArithmeticOverflow {
        field: "dictionary_ordinal",
    })
}

fn append_count(bytes: &mut Vec<u8>, count: u32) {
    bytes.extend_from_slice(&u64::from(count).to_le_bytes());
}

fn append_budgets(bytes: &mut Vec<u8>, budgets: ResidentClearingBudgets) {
    bytes.extend_from_slice(&budgets.max_owners().to_le_bytes());
    bytes.extend_from_slice(&budgets.max_resources().to_le_bytes());
    bytes.extend_from_slice(&budgets.max_scopes().to_le_bytes());
    bytes.extend_from_slice(&budgets.max_draws().to_le_bytes());
    bytes.extend_from_slice(&budgets.max_rows().to_le_bytes());
    bytes.extend_from_slice(&budgets.max_semantic_plan_bytes().to_le_bytes());
    bytes.extend_from_slice(&budgets.max_resident_bytes().to_le_bytes());
    bytes.extend_from_slice(&budgets.max_scratch_bytes().to_le_bytes());
    bytes.extend_from_slice(&budgets.scratch_bytes_per_row().to_le_bytes());
}

fn digest_bytes(bytes: &[u8]) -> SemanticPlanDigest {
    SemanticPlanDigest {
        low: stable_hash64(0xcbf2_9ce4_8422_2325, bytes),
        high: stable_hash64(0x8422_2325_cbf2_9ce4, bytes),
    }
}

fn stable_hash64(seed: u64, bytes: &[u8]) -> u64 {
    bytes.iter().fold(seed, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01b3)
    })
}
