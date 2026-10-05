use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub struct ExecutionContext {
    pub id: u64,
}

pub struct S {
    pub current_context: ExecutionContext,
}

pub open spec fn CallerExecutionContext(s: S) -> ExecutionContext;

pub open spec fn ExecutionContextIsAborted(s: S, ctx: ExecutionContext) -> bool;

pub open spec fn InvocationCompletes(s: S, ret: int) -> bool;

} // verus!
