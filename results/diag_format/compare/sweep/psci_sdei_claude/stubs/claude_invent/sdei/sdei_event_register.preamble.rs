use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub struct S {
    pub sdei_supported: bool,
    pub state_id: nat,
}

pub const SUCCESS: i64 = 0;
pub const NOT_SUPPORTED: i64 = -1;
pub const INVALID_PARAMETERS: i64 = -2;
pub const DENIED: i64 = -3;

pub open spec fn SdeiIsSupported(s: S) -> bool;
pub open spec fn SdeiEventIsValid(s: S, event: i32) -> bool;
pub open spec fn SdeiEntryPointIsValid(s: S, entry_point_address: UInt64, relative_mode: UInt64) -> bool;
pub open spec fn SdeiEventIsShared(s: S, event: i32) -> bool;
pub open spec fn SdeiAffinityIsValid(s: S, affinity: UInt64) -> bool;
pub open spec fn SdeiEventIsRegistered(s: S, event: i32) -> bool;
pub open spec fn SdeiEventIsUnregisterPending(s: S, event: i32) -> bool;
pub open spec fn SdeiEventIsEnabled(s: S, event: i32) -> bool;
pub open spec fn SdeiEventHandlerAddress(s: S, event: i32) -> UInt64;
pub open spec fn SdeiEventHandlerRelativeMode(s: S, event: i32) -> UInt64;
pub open spec fn SdeiEventHandlerArgument(s: S, event: i32) -> UInt64;
pub open spec fn SdeiEventRoutingMode(s: S, event: i32) -> UInt64;
pub open spec fn SdeiEventAffinity(s: S, event: i32) -> UInt64;

} // verus!
