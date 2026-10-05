use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type Int64 = i64;

pub struct S {
    pub sdei_supported: bool,
}

pub const NOT_SUPPORTED: Int64 = -1;
pub const INVALID_PARAMETERS: Int64 = -2;
pub const DENIED: Int64 = -3;

pub const EV_TYPE: UInt32 = 0;
pub const EV_SIGNALED: UInt32 = 1;
pub const EV_PRIORITY: UInt32 = 2;
pub const EV_ROUTING_MODE: UInt32 = 3;
pub const EV_ROUTING_AFF: UInt32 = 4;

pub const RM_ANY: u32 = 0;
pub const RM_PE: u32 = 1;

pub const PRIORITY_NORMAL: u32 = 0;
pub const PRIORITY_CRITICAL: u32 = 1;

pub open spec fn SdeiIsSupported(s: S) -> bool;

pub open spec fn ResultEqual(result: Int64, code: Int64) -> bool;

pub open spec fn IsValidEventNumber(s: S, event: Int32) -> bool;

pub open spec fn IsValidEventInfo(s: S, info: UInt32) -> bool;

pub open spec fn IsSharedEvent(s: S, event: Int32) -> bool;

pub open spec fn IsPrivateEvent(s: S, event: Int32) -> bool;

pub open spec fn IsEventRegistered(s: S, event: Int32) -> bool;

pub open spec fn IsSoftwareSignalable(s: S, event: Int32) -> bool;

pub open spec fn EventRoutingMode(s: S, event: Int32) -> u32;

pub open spec fn EventPriority(s: S, event: Int32) -> u32;

pub open spec fn EventRoutingAffinity(s: S, event: Int32) -> Int64;

pub open spec fn IsMpidrFormat(v: Int64) -> bool;

} // verus!
