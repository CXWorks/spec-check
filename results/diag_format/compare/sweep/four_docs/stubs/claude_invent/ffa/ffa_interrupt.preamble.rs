use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub struct S {
    pub dummy: u32,
}

pub open spec fn FfaInterruptReportsPreemptedRequest(s: S) -> bool;

pub open spec fn FfaInterruptDelegatesToWaitingCallee(s: S) -> bool;

pub open spec fn FfaPreemptedPartitionId(s: S) -> UInt32;

pub open spec fn FfaPreemptedVcpuId(s: S) -> UInt32;

pub open spec fn FfaPendingInterruptId(s: S) -> UInt32;

} // verus!
