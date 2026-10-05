use vstd::prelude::*;

verus! {

pub struct S {
    pub sdei_supported: bool,
    pub state_id: nat,
}

pub const SUCCESS: i64 = 0;
pub const NOT_SUPPORTED: i64 = -1;
pub const INVALID_PARAMETERS: i64 = -2;
pub const DENIED: i64 = -3;

pub open spec fn SdeiIsSupported(s: S) -> bool;

pub open spec fn SdeiEventNumberIsValid(s: S, event: i32) -> bool;

pub open spec fn SdeiEventIsBound(s: S, event: i32) -> bool;

pub open spec fn SdeiEventHandlerUnregisteredOnAllPes(s: S, event: i32) -> bool;

pub open spec fn SdeiBindSlotReturnedToPool(old_s: S, new_s: S, event: i32) -> bool;

pub open spec fn SdeiInterruptConfigRestored(old_s: S, new_s: S, event: i32) -> bool;

pub open spec fn SdeiBoundInterrupt(s: S, event: i32) -> u32;

pub open spec fn SdeiInterruptDisabledAtController(s: S, intr: u32) -> bool;

} // verus!
