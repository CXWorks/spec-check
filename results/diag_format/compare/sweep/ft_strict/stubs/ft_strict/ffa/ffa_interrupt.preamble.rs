use vstd::prelude::*;
verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;
pub type UInt64 = u64;

pub type Instance = u8;
pub type Conduit = u8;
pub type Endpoint = u16;

pub struct S {
    pub dummy: u64,
}

pub spec const NS_PHYSICAL: Instance = 0;
pub spec const NS_VIRTUAL: Instance = 1;
pub spec const S_PHYSICAL: Instance = 2;
pub spec const S_VIRTUAL: Instance = 3;

pub spec const SMC: Conduit = 0;
pub spec const ERET: Conduit = 1;

pub spec const caller: Endpoint = 1;
pub spec const callee: Endpoint = 2;

pub open spec fn instance(s: S) -> Instance;
pub open spec fn conduit(s: S) -> Conduit;

pub open spec fn IsSel1OrSel2Spmc(s: S, e: Endpoint) -> bool;
pub open spec fn IsSpmd(s: S, e: Endpoint) -> bool;
pub open spec fn IsSpmc(s: S, e: Endpoint) -> bool;
pub open spec fn IsSel0(s: S, e: Endpoint) -> bool;
pub open spec fn IsNsInterruptPreemptingSp(s: S, e: Endpoint) -> bool;
pub open spec fn PreemptedSpId(s: S, e: Endpoint) -> UInt16;
pub open spec fn PreemptedSpExecutionContextId(s: S, e: Endpoint) -> UInt16;
pub open spec fn IsBlockedCalleePreemption(s: S, a: Endpoint, b: Endpoint) -> bool;
pub open spec fn IsValidBlockedCombination(s: S, a: Endpoint, b: Endpoint, i: Instance) -> bool;
pub open spec fn PreemptedPartitionId(s: S, e: Endpoint) -> UInt16;
pub open spec fn PreemptedExecutionContextId(s: S, e: Endpoint) -> UInt16;
pub open spec fn IsWaitingCalleeDelegation(s: S, a: Endpoint, b: Endpoint) -> bool;
pub open spec fn IsValidWaitingCombination(s: S, a: Endpoint, b: Endpoint, i: Instance) -> bool;
pub open spec fn PendingInterruptId(s: S, e: Endpoint) -> UInt32;
pub open spec fn ReservedParameterRegistersZero(s: S) -> bool;
pub open spec fn ControlReturnedToCallee(s: S, a: Endpoint, b: Endpoint) -> bool;

} // verus!
