use vstd::prelude::*;
verus! {

pub type UInt64 = u64;
pub type Int64 = i64;
pub type PowerState = u64;

pub struct S {
    pub dummy: u64,
}

pub const HW_ON: Int64 = 0;
pub const HW_OFF: Int64 = 1;
pub const HW_STANDBY: Int64 = 2;
pub const NOT_SUPPORTED: Int64 = -1;
pub const INVALID_PARAMETERS: Int64 = -2;

pub const RUN: PowerState = 0;
pub const POWERDOWN: PowerState = 1;
pub const RETENTION_STANDBY: PowerState = 2;

pub open spec fn IsNodeHwStateImplemented(s: S) -> bool;
pub open spec fn IsValidNode(s: S, target_cpu: UInt64, power_level: UInt64) -> bool;
pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;
pub open spec fn PowerControllerViewOfNode(s: S, target_cpu: UInt64, power_level: UInt64) -> PowerState;

} // verus!
