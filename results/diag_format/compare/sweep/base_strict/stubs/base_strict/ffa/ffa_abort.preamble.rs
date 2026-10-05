use vstd::prelude::*;
verus! {

pub type FunctionId = u32;

pub struct ExecutionContext {
    pub id: u64,
}

pub struct S {
    pub fid: FunctionId,
}

pub open spec fn CallerExecutionContext(s: S) -> ExecutionContext;

pub open spec fn ExecutionContextIsAborted(s: S, ec: ExecutionContext) -> bool;

pub open spec fn InvocationCompletes(fid: FunctionId) -> bool;

} // verus!
