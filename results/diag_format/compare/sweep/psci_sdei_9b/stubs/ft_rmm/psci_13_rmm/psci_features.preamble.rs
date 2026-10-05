use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type Int32 = i32;

pub struct S {
    pub dummy: u64,
}

pub spec const NOT_SUPPORTED: Int32 = -1;

pub open spec fn IsImplementedFunction(s: S, psci_func_id: UInt32) -> bool;

pub open spec fn IsCpuSuspendFid(s: S, psci_func_id: UInt32) -> bool;

pub open spec fn IsSystemOff2Fid(s: S, psci_func_id: UInt32) -> bool;

pub open spec fn UsesExtendedStateIdFormat() -> bool;

pub open spec fn SupportsOsInitiatedMode() -> bool;

} // verus!
