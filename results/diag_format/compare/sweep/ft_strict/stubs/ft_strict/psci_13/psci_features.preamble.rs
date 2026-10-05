use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub caller_is_aarch32: bool,
}

pub spec const NOT_SUPPORTED: Int32 = (-1int) as i32;

pub open spec fn IsImplementedFunction(s: S, func_id: UInt32) -> bool;

pub open spec fn ResultEqual(result: Int32, expected: Int32) -> bool;

pub open spec fn CallerIsAArch32(s: S) -> bool;

pub open spec fn IsSmc64FunctionId(s: S, func_id: UInt32) -> bool;

pub open spec fn IsValidFeatureFlags(s: S, func_id: UInt32, result: Int32) -> bool;

pub open spec fn Bits(value: Int32, hi: int, lo: int) -> int;

pub open spec fn UsesExtendedStateIdFormat() -> bool;

pub open spec fn SupportsOsInitiatedMode() -> bool;

} // verus!
