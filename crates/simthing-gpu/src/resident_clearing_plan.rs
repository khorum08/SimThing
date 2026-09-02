//! Stable resident-plan ABI descriptors and per-tree GPU buffer ownership.
//!
//! Rung 14.2 intentionally defines storage only: there is no shader, pipeline,
//! command encoder, dispatch, readback authority, scoring, or apportionment.

use bytemuck::{Pod, Zeroable};
use simthing_core::{ExecutionIncarnation, GenerationStamp, TreeExecutionBinding, TreeRealmId};
use simthing_kernel::{
    ResidentApportionmentPlan, ResidentClearingPlan, ResidentClearingPlanError, ResidentDrawId,
    ResidentOwnerId, ResidentResourceId, ResidentScopeId, SemanticPlanDigest,
};
use thiserror::Error;
use wgpu::util::DeviceExt;
use wgpu::{Buffer, BufferUsages, Device};

pub const RESIDENT_CLEARING_ABI_VERSION: u32 = 1;
pub const RESIDENT_CLEARING_BUFFER_ALIGNMENT: u64 = 16;

pub const RESIDENT_BUFFER_HEADER: u32 = 0;
pub const RESIDENT_BUFFER_OWNERS: u32 = 1;
pub const RESIDENT_BUFFER_RESOURCES: u32 = 2;
pub const RESIDENT_BUFFER_SCOPES: u32 = 3;
pub const RESIDENT_BUFFER_DRAWS: u32 = 4;
pub const RESIDENT_BUFFER_ROWS: u32 = 5;
pub const RESIDENT_BUFFER_SCRATCH: u32 = 6;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Pod, Zeroable)]
pub struct ResidentClearingHeaderGpu {
    pub abi_version: u32,
    pub generation: u32,
    pub owner_count: u32,
    pub resource_count: u32,
    pub scope_count: u32,
    pub draw_count: u32,
    pub row_count: u32,
    pub scratch_bytes_per_row: u32,
    pub realm_words: [u32; 4],
    pub incarnation_words: [u32; 2],
    pub digest_words: [u32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Pod, Zeroable)]
pub struct ResidentOwnerGpu {
    pub realm_words: [u32; 4],
    pub local_id: u32,
    pub reserved: [u32; 3],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Pod, Zeroable)]
pub struct ResidentSemanticIdGpu {
    pub words: [u32; 2],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Pod, Zeroable)]
pub struct ResidentClearingRowGpu {
    pub owner: u32,
    pub resource: u32,
    pub scope: u32,
    pub draw: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResidentBufferDescriptor {
    kind: u32,
    count: u32,
    stride_bytes: u32,
    logical_bytes: u64,
    allocated_bytes: u64,
}

impl ResidentBufferDescriptor {
    pub const fn kind(self) -> u32 {
        self.kind
    }
    pub const fn count(self) -> u32 {
        self.count
    }
    pub const fn stride_bytes(self) -> u32 {
        self.stride_bytes
    }
    pub const fn logical_bytes(self) -> u64 {
        self.logical_bytes
    }
    pub const fn allocated_bytes(self) -> u64 {
        self.allocated_bytes
    }
}

/// Complete checked physical ABI derived from one immutable semantic plan.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResidentClearingAbi {
    header: ResidentClearingHeaderGpu,
    descriptors: [ResidentBufferDescriptor; 7],
    total_allocated_bytes: u64,
}

impl ResidentClearingAbi {
    /// Finish every count/stride/alignment/byte/scratch/budget check before a
    /// [`wgpu::Buffer`] can be created.
    pub fn from_plan<TResidency>(
        binding: &TreeExecutionBinding<'_, TResidency>,
        plan: &ResidentClearingPlan,
    ) -> Result<Self, ResidentClearingGpuError> {
        let plan_binding = plan.bind_context(binding)?;
        let ranges = plan.ranges();
        let budgets = plan.budgets();
        let header = ResidentClearingHeaderGpu {
            abi_version: RESIDENT_CLEARING_ABI_VERSION,
            generation: plan_binding.generation().get(),
            owner_count: ranges.owners.len(),
            resource_count: ranges.resources.len(),
            scope_count: ranges.scopes.len(),
            draw_count: ranges.draws.len(),
            row_count: ranges.rows.len(),
            scratch_bytes_per_row: budgets.scratch_bytes_per_row(),
            realm_words: realm_words(plan_binding.realm()),
            incarnation_words: u64_words(plan_binding.incarnation().get()),
            digest_words: digest_words(plan_binding.digest()),
        };

        let scratch_logical = u64::from(ranges.rows.len())
            .checked_mul(u64::from(budgets.scratch_bytes_per_row()))
            .ok_or(ResidentClearingGpuError::ArithmeticOverflow {
                field: "row_count*scratch_bytes_per_row",
            })?;
        if scratch_logical > budgets.max_scratch_bytes() {
            return Err(ResidentClearingGpuError::ScratchBudgetExceeded {
                required: scratch_logical,
                admitted: budgets.max_scratch_bytes(),
            });
        }

        let descriptors = [
            descriptor::<ResidentClearingHeaderGpu>(RESIDENT_BUFFER_HEADER, 1)?,
            descriptor::<ResidentOwnerGpu>(RESIDENT_BUFFER_OWNERS, ranges.owners.len())?,
            descriptor::<ResidentSemanticIdGpu>(RESIDENT_BUFFER_RESOURCES, ranges.resources.len())?,
            descriptor::<ResidentSemanticIdGpu>(RESIDENT_BUFFER_SCOPES, ranges.scopes.len())?,
            descriptor::<ResidentSemanticIdGpu>(RESIDENT_BUFFER_DRAWS, ranges.draws.len())?,
            descriptor::<ResidentClearingRowGpu>(RESIDENT_BUFFER_ROWS, ranges.rows.len())?,
            byte_descriptor(RESIDENT_BUFFER_SCRATCH, ranges.rows.len(), scratch_logical)?,
        ];
        for descriptor in descriptors {
            usize::try_from(descriptor.allocated_bytes).map_err(|_| {
                ResidentClearingGpuError::AbiNarrowing {
                    field: "host_allocation_bytes",
                    value: descriptor.allocated_bytes,
                }
            })?;
        }
        let total_allocated_bytes = descriptors.iter().try_fold(0_u64, |total, descriptor| {
            total.checked_add(descriptor.allocated_bytes).ok_or(
                ResidentClearingGpuError::ArithmeticOverflow {
                    field: "total_resident_bytes",
                },
            )
        })?;
        if total_allocated_bytes > budgets.max_resident_bytes() {
            return Err(ResidentClearingGpuError::ResidentBudgetExceeded {
                required: total_allocated_bytes,
                admitted: budgets.max_resident_bytes(),
            });
        }
        Ok(Self {
            header,
            descriptors,
            total_allocated_bytes,
        })
    }

    pub const fn header(&self) -> ResidentClearingHeaderGpu {
        self.header
    }

    pub const fn descriptors(&self) -> &[ResidentBufferDescriptor; 7] {
        &self.descriptors
    }

    pub const fn total_allocated_bytes(&self) -> u64 {
        self.total_allocated_bytes
    }

    pub fn descriptor(&self, kind: u32) -> Option<ResidentBufferDescriptor> {
        self.descriptors
            .iter()
            .copied()
            .find(|descriptor| descriptor.kind == kind)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResidentClearingBufferOwner {
    realm: TreeRealmId,
    incarnation: ExecutionIncarnation,
    generation: GenerationStamp,
    digest: SemanticPlanDigest,
}

impl ResidentClearingBufferOwner {
    pub const fn realm(self) -> TreeRealmId {
        self.realm
    }
    pub const fn incarnation(self) -> ExecutionIncarnation {
        self.incarnation
    }
    pub const fn generation(self) -> GenerationStamp {
        self.generation
    }
    pub const fn digest(self) -> SemanticPlanDigest {
        self.digest
    }
}

/// Typed proof that only transient resident header state advanced.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResidentGenerationAdvance {
    previous: GenerationStamp,
    current: GenerationStamp,
    digest: SemanticPlanDigest,
}

impl ResidentGenerationAdvance {
    pub const fn previous(self) -> GenerationStamp {
        self.previous
    }
    pub const fn current(self) -> GenerationStamp {
        self.current
    }
    pub const fn digest(self) -> SemanticPlanDigest {
        self.digest
    }
}

/// Per-tree owner of all buffers in one resident clearing plan.
///
/// The buffers are private and the type is neither `Clone` nor globally
/// registered, so another tree cannot alias this semantic owner accidentally.
pub struct ResidentClearingBuffers {
    owner: ResidentClearingBufferOwner,
    abi: ResidentClearingAbi,
    header: Buffer,
    owners: Buffer,
    resources: Buffer,
    scopes: Buffer,
    draws: Buffer,
    rows: Buffer,
    scratch: Buffer,
}

impl ResidentClearingBuffers {
    pub fn allocate<TResidency>(
        device: &Device,
        binding: &TreeExecutionBinding<'_, TResidency>,
        plan: &ResidentClearingPlan,
    ) -> Result<Self, ResidentClearingGpuError> {
        // Binding and all budget/arithmetic checks precede the first allocation.
        let abi = ResidentClearingAbi::from_plan(binding, plan)?;
        let device_max = device.limits().max_buffer_size;
        if let Some(descriptor) = abi
            .descriptors()
            .iter()
            .find(|descriptor| descriptor.allocated_bytes() > device_max)
        {
            return Err(ResidentClearingGpuError::DeviceBufferLimitExceeded {
                kind: descriptor.kind(),
                required: descriptor.allocated_bytes(),
                admitted: device_max,
            });
        }
        let context = binding.context();
        let owner = ResidentClearingBufferOwner {
            realm: context.realm(),
            incarnation: context.incarnation(),
            generation: binding.generation(),
            digest: plan.digest(),
        };

        let owners: Vec<_> = plan
            .dictionaries()
            .owners()
            .iter()
            .copied()
            .map(owner_gpu)
            .collect();
        let resources: Vec<_> = plan
            .dictionaries()
            .resources()
            .iter()
            .copied()
            .map(|id| semantic_id_gpu(id.get()))
            .collect();
        let scopes: Vec<_> = plan
            .dictionaries()
            .scopes()
            .iter()
            .copied()
            .map(|id| semantic_id_gpu(id.get()))
            .collect();
        let draws: Vec<_> = plan
            .dictionaries()
            .draws()
            .iter()
            .copied()
            .map(|id| semantic_id_gpu(id.get()))
            .collect();
        let rows: Vec<_> = plan
            .rows()
            .iter()
            .copied()
            .map(|row| ResidentClearingRowGpu {
                owner: row.owner().get(),
                resource: row.resource().get(),
                scope: row.scope().get(),
                draw: row.draw().get(),
            })
            .collect();

        let header = create_init(
            device,
            "resident_clearing_header",
            bytemuck::bytes_of(&abi.header),
            abi.descriptor(RESIDENT_BUFFER_HEADER)
                .expect("the fixed ABI has one header descriptor")
                .allocated_bytes(),
        );
        let owners = create_init(
            device,
            "resident_clearing_owners",
            bytemuck::cast_slice(&owners),
            abi.descriptor(RESIDENT_BUFFER_OWNERS)
                .expect("the fixed ABI has one owner descriptor")
                .allocated_bytes(),
        );
        let resources = create_init(
            device,
            "resident_clearing_resources",
            bytemuck::cast_slice(&resources),
            abi.descriptor(RESIDENT_BUFFER_RESOURCES)
                .expect("the fixed ABI has one resource descriptor")
                .allocated_bytes(),
        );
        let scopes = create_init(
            device,
            "resident_clearing_scopes",
            bytemuck::cast_slice(&scopes),
            abi.descriptor(RESIDENT_BUFFER_SCOPES)
                .expect("the fixed ABI has one scope descriptor")
                .allocated_bytes(),
        );
        let draws = create_init(
            device,
            "resident_clearing_draws",
            bytemuck::cast_slice(&draws),
            abi.descriptor(RESIDENT_BUFFER_DRAWS)
                .expect("the fixed ABI has one draw descriptor")
                .allocated_bytes(),
        );
        let rows = create_init(
            device,
            "resident_clearing_rows",
            bytemuck::cast_slice(&rows),
            abi.descriptor(RESIDENT_BUFFER_ROWS)
                .expect("the fixed ABI has one row descriptor")
                .allocated_bytes(),
        );
        let scratch_descriptor = abi
            .descriptor(RESIDENT_BUFFER_SCRATCH)
            .expect("the fixed ABI always has one scratch descriptor");
        let scratch = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("resident_clearing_scratch"),
            size: scratch_descriptor.allocated_bytes(),
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Ok(Self {
            owner,
            abi,
            header,
            owners,
            resources,
            scopes,
            draws,
            rows,
            scratch,
        })
    }

    /// Advance N -> N+1 in transient owner/header state only.
    ///
    /// This method creates no buffer and does not rebuild or upload semantic
    /// dictionaries. The future 14.3 dispatch path may write the updated POD
    /// header to the already-owned header buffer.
    pub fn advance_generation<TResidency>(
        &mut self,
        binding: &TreeExecutionBinding<'_, TResidency>,
        plan: &ResidentClearingPlan,
    ) -> Result<ResidentGenerationAdvance, ResidentClearingGpuError> {
        let plan_binding = plan.bind_context(binding)?;
        if plan_binding.realm() != self.owner.realm
            || plan_binding.incarnation() != self.owner.incarnation
            || plan_binding.digest() != self.owner.digest
        {
            return Err(ResidentClearingGpuError::ResidentOwnerMismatch);
        }
        let previous = self.owner.generation;
        let expected = previous
            .get()
            .checked_add(1)
            .ok_or(ResidentClearingGpuError::GenerationOverflow)?;
        let current = plan_binding.generation();
        if current.get() != expected {
            return Err(ResidentClearingGpuError::GenerationAdvanceOutOfSequence {
                previous,
                observed: current,
            });
        }
        self.owner.generation = current;
        self.abi.header.generation = current.get();
        Ok(ResidentGenerationAdvance {
            previous,
            current,
            digest: self.owner.digest,
        })
    }

    pub const fn owner(&self) -> ResidentClearingBufferOwner {
        self.owner
    }

    pub const fn abi(&self) -> &ResidentClearingAbi {
        &self.abi
    }

    pub const fn header_buffer(&self) -> &Buffer {
        &self.header
    }
    pub const fn owner_buffer(&self) -> &Buffer {
        &self.owners
    }
    pub const fn resource_buffer(&self) -> &Buffer {
        &self.resources
    }
    pub const fn scope_buffer(&self) -> &Buffer {
        &self.scopes
    }
    pub const fn draw_buffer(&self) -> &Buffer {
        &self.draws
    }
    pub const fn row_buffer(&self) -> &Buffer {
        &self.rows
    }
    pub const fn scratch_buffer(&self) -> &Buffer {
        &self.scratch
    }

    /// Borrow the immutable semantic-row table and admitted scratch for one
    /// exact-apportionment plan. This is a buffer-ownership check, not an
    /// economic adapter: the canonical product is written directly in scratch.
    pub fn apportionment_buffers(
        &self,
        plan: &ResidentApportionmentPlan,
    ) -> Result<(&Buffer, &Buffer), ResidentClearingGpuError> {
        if plan.semantic_digest() != self.owner.digest {
            return Err(ResidentClearingGpuError::ApportionmentPlanDigestMismatch);
        }
        if plan.row_count() != self.abi.header.row_count {
            return Err(
                ResidentClearingGpuError::ApportionmentPlanRowCountMismatch {
                    plan: plan.row_count(),
                    resident: self.abi.header.row_count,
                },
            );
        }
        Ok((&self.rows, &self.scratch))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum ResidentClearingGpuError {
    #[error(transparent)]
    Plan(#[from] ResidentClearingPlanError),
    #[error("resident clearing buffer arithmetic overflow in {field}")]
    ArithmeticOverflow { field: &'static str },
    #[error("resident clearing scratch requires {required} bytes, admitted {admitted}")]
    ScratchBudgetExceeded { required: u64, admitted: u64 },
    #[error("resident clearing buffers require {required} bytes, admitted {admitted}")]
    ResidentBudgetExceeded { required: u64, admitted: u64 },
    #[error("resident clearing ABI cannot represent {field}={value}")]
    AbiNarrowing { field: &'static str, value: u64 },
    #[error(
        "resident clearing buffer kind {kind} requires {required} bytes, device admits {admitted}"
    )]
    DeviceBufferLimitExceeded {
        kind: u32,
        required: u64,
        admitted: u64,
    },
    #[error("resident clearing generation overflow")]
    GenerationOverflow,
    #[error("resident generation advance must be N -> N+1: previous {previous:?}, observed {observed:?}")]
    GenerationAdvanceOutOfSequence {
        previous: GenerationStamp,
        observed: GenerationStamp,
    },
    #[error("resident clearing generation advance belongs to a different owner")]
    ResidentOwnerMismatch,
    #[error("resident exact-apportionment plan belongs to a different semantic plan")]
    ApportionmentPlanDigestMismatch,
    #[error("resident exact-apportionment row count {plan} differs from resident rows {resident}")]
    ApportionmentPlanRowCountMismatch { plan: u32, resident: u32 },
}

fn descriptor<T>(
    kind: u32,
    count: u32,
) -> Result<ResidentBufferDescriptor, ResidentClearingGpuError> {
    let stride = u64::try_from(std::mem::size_of::<T>()).map_err(|_| {
        ResidentClearingGpuError::ArithmeticOverflow {
            field: "size_of ABI row",
        }
    })?;
    let logical = u64::from(count).checked_mul(stride).ok_or(
        ResidentClearingGpuError::ArithmeticOverflow {
            field: "count*stride",
        },
    )?;
    descriptor_parts(kind, count, stride, logical)
}

fn byte_descriptor(
    kind: u32,
    count: u32,
    logical: u64,
) -> Result<ResidentBufferDescriptor, ResidentClearingGpuError> {
    let stride = if count == 0 {
        0
    } else {
        logical / u64::from(count)
    };
    descriptor_parts(kind, count, stride, logical)
}

fn descriptor_parts(
    kind: u32,
    count: u32,
    stride: u64,
    logical: u64,
) -> Result<ResidentBufferDescriptor, ResidentClearingGpuError> {
    let stride_bytes =
        u32::try_from(stride).map_err(|_| ResidentClearingGpuError::AbiNarrowing {
            field: "stride_bytes",
            value: stride,
        })?;
    let allocated_bytes = align_up(logical, RESIDENT_CLEARING_BUFFER_ALIGNMENT)?;
    Ok(ResidentBufferDescriptor {
        kind,
        count,
        stride_bytes,
        logical_bytes: logical,
        allocated_bytes,
    })
}

fn align_up(value: u64, alignment: u64) -> Result<u64, ResidentClearingGpuError> {
    let mask = alignment - 1;
    value
        .checked_add(mask)
        .map(|sum| sum & !mask)
        .ok_or(ResidentClearingGpuError::ArithmeticOverflow { field: "alignment" })
}

fn create_init(device: &Device, label: &'static str, bytes: &[u8], allocated_bytes: u64) -> Buffer {
    let mut padded = bytes.to_vec();
    padded.resize(
        usize::try_from(allocated_bytes).expect("ABI admission checked host-size representation"),
        0,
    );
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some(label),
        contents: &padded,
        usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
    })
}

fn realm_words(realm: TreeRealmId) -> [u32; 4] {
    let bytes = realm.canonical_bytes();
    [
        u32::from_le_bytes(bytes[0..4].try_into().expect("fixed realm slice")),
        u32::from_le_bytes(bytes[4..8].try_into().expect("fixed realm slice")),
        u32::from_le_bytes(bytes[8..12].try_into().expect("fixed realm slice")),
        u32::from_le_bytes(bytes[12..16].try_into().expect("fixed realm slice")),
    ]
}

fn u64_words(value: u64) -> [u32; 2] {
    let bytes = value.to_le_bytes();
    [
        u32::from_le_bytes(bytes[..4].try_into().expect("fixed low word")),
        u32::from_le_bytes(bytes[4..].try_into().expect("fixed high word")),
    ]
}

fn digest_words(digest: SemanticPlanDigest) -> [u32; 4] {
    let low = u64_words(digest.low());
    let high = u64_words(digest.high());
    [low[0], low[1], high[0], high[1]]
}

fn owner_gpu(owner: ResidentOwnerId) -> ResidentOwnerGpu {
    let identity = owner.identity();
    ResidentOwnerGpu {
        realm_words: realm_words(identity.realm()),
        local_id: identity.local().raw(),
        reserved: [0; 3],
    }
}

fn semantic_id_gpu(value: u64) -> ResidentSemanticIdGpu {
    ResidentSemanticIdGpu {
        words: u64_words(value),
    }
}

#[allow(dead_code)]
fn _typed_axis_compile_guards(
    _resource: ResidentResourceId,
    _scope: ResidentScopeId,
    _draw: ResidentDrawId,
) {
}
