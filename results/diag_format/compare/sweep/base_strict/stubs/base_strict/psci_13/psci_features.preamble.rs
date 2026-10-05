use vstd::prelude::*;

verus! {

pub type Int32 = i32;

pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub const NOT_SUPPORTED: Int32 = -1;

pub open spec fn IsImplementedFunction(func_id: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn CallerIsAArch32() -> bool;

pub open spec fn IsSmc64FunctionId(func_id: UInt32) -> bool;

pub open spec fn IsValidFeatureFlags(func_id: UInt32, result: Int32) -> bool;

pub open spec fn Bits(value: Int32, hi: int, lo: int) -> int;

pub open spec fn UsesExtendedStateIdFormat() -> bool;

pub open spec fn SupportsOsInitiatedMode() -> bool;

} // verus!
