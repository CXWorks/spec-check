use vstd::prelude::*;

verus! {

pub struct S {
    pub sdei_supported: bool,
    pub shared_slots: u64,
    pub private_slots: u64,
    pub relative_mode: bool,
}

pub const NOT_SUPPORTED: i64 = -1;
pub const INVALID_PARAMETERS: i64 = -2;

pub open spec fn SdeiIsSupported(s: S) -> bool;

pub open spec fn SdeiSharedEventSlotCount(s: S) -> u64;

pub open spec fn SdeiPrivateEventSlotCount(s: S) -> u64;

pub open spec fn SdeiRelativeModeSupported(s: S) -> bool;

} // verus!
