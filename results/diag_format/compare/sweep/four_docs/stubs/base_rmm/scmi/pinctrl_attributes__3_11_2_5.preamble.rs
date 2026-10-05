use vstd::prelude::*;

verus! {

pub type Int32 = i32;

pub type UInt8 = u8;

pub struct UInt32 {
    pub value: u32,
}

impl UInt32 {
    pub open spec fn spec_index(self, i: int) -> int;
}

pub type PinctrlNameBytes = Seq<UInt8>;

pub struct S {
    pub num_functions: nat,
    pub num_groups: nat,
    pub num_pins: nat,
}

pub spec const SUCCESS: Int32 = 0;

pub spec const NOT_FOUND: Int32 = -4;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn PinctrlObjectExists(flags: UInt8, identifier: UInt32) -> bool;

pub open spec fn PinctrlNameLength(flags: UInt8, identifier: UInt32) -> int;

pub open spec fn FunctionSupportsGpio(identifier: UInt32) -> int;

pub open spec fn IsPinOnlyFunction(identifier: UInt32) -> bool;

pub open spec fn GroupPinCount(identifier: UInt32) -> int;

pub open spec fn FunctionPinCount(identifier: UInt32) -> int;

pub open spec fn FunctionGroupCount(identifier: UInt32) -> int;

pub open spec fn PinctrlName(flags: UInt8, identifier: UInt32) -> PinctrlNameBytes;

pub open spec fn flags_from_selector_and_identifier(s: S, result: Int32, attributes: UInt32) -> UInt8;

pub open spec fn identifier_from_selector_and_identifier(s: S, result: Int32, attributes: UInt32) -> UInt32;

pub open spec fn count_functions_with_gpio_support(s: S) -> int;

pub open spec fn lower_15_bytes_of(name: PinctrlNameBytes) -> PinctrlNameBytes;

} // verus!
