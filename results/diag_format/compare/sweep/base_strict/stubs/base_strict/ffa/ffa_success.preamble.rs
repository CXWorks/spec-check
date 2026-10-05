use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub struct S {
    pub conduit_is_smc: bool,
    pub instance_is_non_secure_virtual: bool,
}

pub struct Invocation {
    pub id: nat,
}

pub open spec fn PreviousInvocation() -> Invocation;

pub open spec fn ResultsReturnedToInvoker(inv: Invocation, results: [UInt32; 8]) -> bool;

pub open spec fn ConduitIsSmc(s: S) -> bool;

pub open spec fn InstanceIsNonSecureVirtual(s: S) -> bool;

pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt32;

pub open spec fn ResultsDeliveredToVcpu(vm_id: UInt32, vcpu_id: UInt32, results: [UInt32; 8]) -> bool;

pub open spec fn AnyResultIs64Bit(results: [UInt32; 8]) -> bool;

pub open spec fn CallerImplementsOnlySmc64Fids() -> bool;

} // verus!
