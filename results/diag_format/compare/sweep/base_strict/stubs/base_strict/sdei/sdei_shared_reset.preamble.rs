use vstd::prelude::*;

verus! {

pub type Int64 = i64;

pub type UInt32 = u32;

pub const TRUE: bool = true;

pub const FALSE: bool = false;

pub const SUCCESS: Int64 = 0;

pub const NOT_SUPPORTED: Int64 = -1;

pub const INVALID_PARAMETERS: Int64 = -2;

pub const DENIED: Int64 = -3;

pub struct SdeiEvent {
    pub event_num: UInt32,
    pub handler_running: bool,
}

pub struct Interrupt {
    pub intid: UInt32,
}

pub struct S {
    pub sdei_supported: bool,
}

pub open spec fn IsSdeiSupported() -> bool;

pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;

pub open spec fn IsSharedEvent(e: SdeiEvent) -> bool;

pub open spec fn IsPrivateEvent(e: SdeiEvent) -> bool;

pub open spec fn IsInterruptBoundEvent(e: SdeiEvent) -> bool;

pub open spec fn IsEventRegistered(e: SdeiEvent) -> bool;

pub open spec fn IsInterruptBoundToEvent(i: Interrupt) -> bool;

pub open spec fn SharedAuxiliaryInfoCleared() -> bool;

pub open spec fn PrivateEventStateUnchanged(e: SdeiEvent) -> bool;

} // verus!
