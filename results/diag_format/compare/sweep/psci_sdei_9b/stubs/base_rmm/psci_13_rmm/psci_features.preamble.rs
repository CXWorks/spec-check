use vstd::prelude::*;

verus! {

pub type Int32 = i32;

pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub spec const NOT_SUPPORTED: Int32 = -1;

pub open spec fn IsImplementedFunction(psci_func_id: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn IsCpuSuspendFid(psci_func_id: UInt32) -> bool;

pub open spec fn IsSystemOff2Fid(psci_func_id: UInt32) -> bool;

pub open spec fn UsesExtendedStateIdFormat() -> bool;

pub open spec fn SupportsOsInitiatedMode() -> bool;

} // verus!
