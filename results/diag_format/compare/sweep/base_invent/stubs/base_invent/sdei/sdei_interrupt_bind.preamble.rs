use vstd::prelude::*;

verus! {

pub type int64 = i64;
pub type uint32 = u32;

pub struct Realm {
    pub sdei_event_bound: Map<uint32, Option<int64>>,
}

pub struct S {
    pub realms: Map<uint32, Realm>,
}

pub const SDEI_NOT_SUPPORTED: int64 = -1;
pub const SDEI_INVALID_PARAMETERS: int64 = -2;
pub const SDEI_DENIED: int64 = -3;
pub const SDEI_OUT_OF_RESOURCE: int64 = -10;

#[allow(non_upper_case_globals)]
pub const interrupt: uint32 = 0;
#[allow(non_upper_case_globals)]
pub const rd: uint32 = 0;
#[allow(non_upper_case_globals)]
pub const new_event_number: int64 = 1;

pub open spec fn ResultEqual(result: int64, code: int64) -> bool;

pub open spec fn SdeiSupported(s: S) -> bool;

pub open spec fn IsValidInterruptNumber(s: S, intr: uint32) -> bool;

pub open spec fn IsInterruptOwnedByClient(s: S, intr: uint32) -> bool;

pub open spec fn IsInterruptInactive(s: S, intr: uint32) -> bool;

pub open spec fn HasAvailableBindSlot(s: S) -> bool;

pub open spec fn InterruptBound(s: S, intr: uint32) -> bool;

pub open spec fn InterruptEnabled(s: S, intr: uint32) -> bool;

pub open spec fn InterruptActive(s: S, intr: uint32) -> bool;

pub open spec fn RealmAt(s: S, r: uint32) -> Realm;

} // verus!
