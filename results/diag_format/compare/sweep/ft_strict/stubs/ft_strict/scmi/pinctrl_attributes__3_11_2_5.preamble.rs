use vstd::prelude::*;

verus! {

pub type UInt8 = u8;
pub type UInt32 = u32;
pub type Int32 = i32;

pub enum RsiCommandReturnCode {
    RsiError,
    RsiIncomplete,
}

pub struct S {
    pub dummy: int,
}

pub spec const RSI_SUCCESS: Result<(), RsiCommandReturnCode> = Ok(());

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_FOUND: Int32 = (-4int) as i32;

pub open spec fn Bits(value: UInt32, hi: int, lo: int) -> UInt32;

pub open spec fn ResultEqual(result: Result<(), RsiCommandReturnCode>, code: Int32) -> bool;

pub open spec fn EntityExists(s: S, identifier: UInt32, kind: UInt32) -> bool;

pub open spec fn EntityNameLength(s: S, identifier: UInt32, kind: UInt32) -> nat;

pub open spec fn ExtendedNameSupported(s: S, identifier: UInt32, kind: UInt32) -> bool;

pub open spec fn FunctionSupportsGpio(s: S, identifier: UInt32) -> bool;

pub open spec fn IsOnlyGpioFunctionOfAssociatedPinsAndGroups(s: S, identifier: UInt32) -> bool;

pub open spec fn IsPinOnlyFunction(s: S, identifier: UInt32) -> bool;

pub open spec fn GroupPinCount(s: S, identifier: UInt32) -> UInt32;

pub open spec fn FunctionSupportingPinCount(s: S, identifier: UInt32) -> UInt32;

pub open spec fn FunctionSupportingGroupCount(s: S, identifier: UInt32) -> UInt32;

pub open spec fn EntityName(s: S, identifier: UInt32, kind: UInt32) -> Seq<UInt8>;

pub open spec fn NameFieldHoldsNullTerminatedName(name: [UInt8; 16], entity_name: Seq<UInt8>) -> bool;

pub open spec fn NameFieldHoldsLower15BytesNullTerminated(name: [UInt8; 16], entity_name: Seq<UInt8>) -> bool;

} // verus!
