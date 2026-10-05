use vstd::prelude::*;

verus! {

pub type RsiCommandReturnCode = u64;
pub type ConduitKind = u64;
pub type ErrorCode = u64;
pub type PartitionId = u16;
pub type VcpuId = u16;

pub struct S {
    pub dummy: u64,
}

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;

pub const SMC: ConduitKind = 1;
pub const HVC: ConduitKind = 2;

pub spec const error_code: ErrorCode = 3;
pub spec const target_id: PartitionId = 4;
pub spec const target_vcpu: VcpuId = 5;

pub open spec fn ErrorCodeReturnedToPreviousCaller(code: ErrorCode) -> bool;

pub open spec fn IsNonSecureVirtualInstance(s: S) -> bool;

pub open spec fn ConduitIs(s: S, conduit: ConduitKind) -> bool;

pub open spec fn ErrorCodeDeliveredTo(id: PartitionId, vcpu: VcpuId, code: ErrorCode) -> bool;

} // verus!
