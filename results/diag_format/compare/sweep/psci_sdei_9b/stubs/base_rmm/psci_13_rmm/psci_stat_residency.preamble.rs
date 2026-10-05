use vstd::prelude::*;

verus! {

pub type UInt = u64;

pub type PowerState = u64;

pub type NodeId = u64;

pub type LocalState = u64;

pub type FunctionId = u32;

pub type CpuId = u64;

pub struct S {
    pub dummy: u64,
}

pub const NOT_SUPPORTED: u64 = 0xFFFF_FFFF_FFFF_FFFF;

pub open spec fn StatFunctionsImplemented(s: S) -> bool;

pub open spec fn ResultEqual(result: UInt, value: u64) -> bool;

pub open spec fn IsPresentNode(s: S, cpu: CpuId) -> bool;

pub open spec fn target_cpu(s: S) -> CpuId;

pub open spec fn NodeSupportsState(s: S, node: NodeId, state: LocalState) -> bool;

pub open spec fn StatNode(s: S, ps: PowerState) -> NodeId;

pub open spec fn StatLocalState(ps: PowerState) -> LocalState;

pub open spec fn power_state(s: S) -> PowerState;

pub open spec fn IsStatResidency(f: FunctionId) -> bool;

pub open spec fn fid(s: S) -> FunctionId;

pub open spec fn StateResidencyUs(s: S, node: NodeId, state: LocalState) -> u64;

pub open spec fn ResultBits(f: FunctionId) -> u32;

} // verus!
