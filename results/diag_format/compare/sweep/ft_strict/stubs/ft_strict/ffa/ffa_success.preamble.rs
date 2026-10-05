use vstd::prelude::*;
verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;
pub type UInt64 = u64;

pub struct S {
    pub dummy: int,
}

pub struct Invocation {
    pub id: int,
}

pub const fid: UInt32 = 0x84000000;

pub uninterp spec fn PreviousInvocation(s: S) -> Invocation;

pub uninterp spec fn ResultsReturnedToInvoker(s: S, inv: Invocation, results: [UInt32; 6]) -> bool;

pub uninterp spec fn ConduitIsSmc(s: S) -> bool;

pub uninterp spec fn InstanceIsNonSecureVirtual(s: S) -> bool;

pub uninterp spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt32;

pub uninterp spec fn ResultsDeliveredToVcpu(s: S, endpoint_id: UInt32, vcpu_id: UInt32, results: [UInt32; 6]) -> bool;

pub uninterp spec fn AnyResultIs64Bit(results: [UInt32; 6]) -> bool;

pub uninterp spec fn CallerImplementsOnlySmc64Fids() -> bool;

} // verus!
