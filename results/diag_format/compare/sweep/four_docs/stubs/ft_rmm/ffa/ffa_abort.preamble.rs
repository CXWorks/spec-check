use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub type ContextId = u64;

pub type ContextState = u64;

pub struct S {
    pub dummy: u64,
}

pub spec const caller: ContextId = 0;

pub spec const ABORTED: ContextState = 1;

pub spec const RUNNING: ContextState = 2;

pub spec const WAITING: ContextState = 3;

pub spec const BLOCKED: ContextState = 4;

pub spec const PREEMPTED: ContextState = 5;

pub open spec fn ExecutionContextState(s: S, c: ContextId) -> ContextState;

pub open spec fn InvocationDoesNotComplete() -> bool;

} // verus!
