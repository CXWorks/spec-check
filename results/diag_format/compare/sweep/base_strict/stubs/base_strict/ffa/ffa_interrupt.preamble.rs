use vstd::prelude::*;

verus! {

pub enum RsiCommandReturnCode {
    RsiError,
    RsiErrorInput,
    RsiErrorState,
    RsiIncomplete,
}

pub type UInt64 = u64;
pub type Instance = u64;
pub type Conduit = u64;
pub type Endpoint = u64;

pub spec const RSI_SUCCESS: Result<(), RsiCommandReturnCode> = Ok(());

pub spec const NS_PHYSICAL: Instance = 0;
pub spec const NS_VIRTUAL: Instance = 1;
pub spec const S_PHYSICAL: Instance = 2;
pub spec const S_VIRTUAL: Instance = 3;

pub spec const SMC: Conduit = 10;
pub spec const ERET: Conduit = 11;
pub spec const HVC: Conduit = 12;

pub struct S {
    pub fid: UInt64,
    pub instance: Instance,
    pub conduit: Conduit,
    pub caller: Endpoint,
    pub callee: Endpoint,
    pub endpoint_id: UInt64,
    pub vcpu_id: UInt64,
    pub interrupt_id: UInt64,
}

pub open spec fn IsSel1OrSel2Spmc(e: Endpoint) -> bool;
pub open spec fn IsSpmd(e: Endpoint) -> bool;
pub open spec fn IsSpmc(e: Endpoint) -> bool;
pub open spec fn IsSel0(e: Endpoint) -> bool;
pub open spec fn IsNsInterruptPreemptingSp(e: Endpoint) -> bool;
pub open spec fn PreemptedSpId(e: Endpoint) -> UInt64;
pub open spec fn PreemptedSpExecutionContextId(e: Endpoint) -> UInt64;
pub open spec fn IsBlockedCalleePreemption(caller: Endpoint, callee: Endpoint) -> bool;
pub open spec fn IsValidBlockedCombination(caller: Endpoint, callee: Endpoint, instance: Instance) -> bool;
pub open spec fn PreemptedPartitionId(e: Endpoint) -> UInt64;
pub open spec fn PreemptedExecutionContextId(e: Endpoint) -> UInt64;
pub open spec fn IsWaitingCalleeDelegation(caller: Endpoint, callee: Endpoint) -> bool;
pub open spec fn IsValidWaitingCombination(caller: Endpoint, callee: Endpoint, instance: Instance) -> bool;
pub open spec fn PendingInterruptId(e: Endpoint) -> UInt64;
pub open spec fn ReservedParameterRegistersZero(s: S) -> bool;
pub open spec fn ControlReturnedToCallee(caller: Endpoint, callee: Endpoint) -> bool;

} // verus!
