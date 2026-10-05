use vstd::prelude::*;

verus! {

pub type Caller = u32;
pub type Callee = u32;
pub type Result = u32;
pub type FunctionId = u32;
pub type Instance = u32;

pub struct S {
    pub dummy: u64,
}

pub spec const RETRY: Result = 1;
pub spec const DENIED: Result = 2;
pub spec const NOT_SUPPORTED: Result = 3;
pub spec const FFA_MSG_SEND: Result = 4;
pub spec const FFA_ERROR: Result = 5;

pub spec const FFA_MSG_POLL: FunctionId = 100;

#[allow(non_upper_case_globals)]
pub spec const ffa_instance: Instance = 200;

pub open spec fn MessageAvailableInRxBuffer(s: S, caller: Caller) -> bool;

pub open spec fn CalleeInStateToHandleRequest(s: S, callee: Callee) -> bool;

pub open spec fn IsImplementedAtInstance(s: S, func: FunctionId, instance: Instance) -> bool;

pub open spec fn ResultEqual(r1: Result, r2: Result) -> bool;

} // verus!
