use vstd::prelude::*;

verus! {

pub type VcpuId = u64;

pub type ExecState = u64;

pub struct S {
    pub dummy: u64,
}

pub spec const caller: VcpuId = 0;

pub spec const RUNNING: ExecState = 0;
pub spec const WAITING: ExecState = 1;
pub spec const BLOCKED: ExecState = 2;
pub spec const ABORTED: ExecState = 3;

pub open spec fn ExecutionContextState(s: S, ec: VcpuId) -> ExecState;

pub open spec fn InvocationDoesNotComplete() -> bool;

} // verus!
