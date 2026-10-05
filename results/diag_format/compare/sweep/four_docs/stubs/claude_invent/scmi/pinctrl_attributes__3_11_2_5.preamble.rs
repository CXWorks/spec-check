use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub type ScmiStatus = i32;

pub const SUCCESS: ScmiStatus = 0;
pub const NOT_FOUND: ScmiStatus = -4;

pub struct S {
    pub dummy: int,
}

pub open spec fn PinctrlExists(s: S, kind: int, id: int) -> bool;

pub open spec fn PinctrlNameIsExtended(s: S, kind: int, id: int) -> bool;

pub open spec fn PinctrlFunctionSupportsGpio(s: S, id: int) -> bool;

pub open spec fn PinctrlFunctionIsPinOnly(s: S, id: int) -> bool;

pub open spec fn PinctrlGroupPinCount(s: S, id: int) -> int;

pub open spec fn PinctrlFunctionPinCount(s: S, id: int) -> int;

pub open spec fn PinctrlFunctionGroupCount(s: S, id: int) -> int;

pub open spec fn PinctrlName(s: S, kind: int, id: int) -> Seq<u8>;

} // verus!
