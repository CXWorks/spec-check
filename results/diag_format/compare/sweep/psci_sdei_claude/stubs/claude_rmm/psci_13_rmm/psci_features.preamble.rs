use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: u64,
}

pub const NOT_SUPPORTED: Int32 = -1i32;

pub open spec fn IsImplementedFunction(fid: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn IsCpuSuspendFid(fid: UInt32) -> bool;

pub open spec fn IsSystemOff2Fid(fid: UInt32) -> bool;

pub open spec fn UsesExtendedStateIdFormat() -> bool;

pub open spec fn SupportsOsInitiatedMode() -> bool;

} // verus!
