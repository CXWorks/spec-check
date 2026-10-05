use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;

pub struct S {
    pub regs: Seq<UInt64>,
    pub implemented: Set<UInt32>,
}

pub const FFA_SUCCESS: UInt32 = 0x84000061u32;
pub const NOT_SUPPORTED: UInt32 = 0xFFFFFFFFu32;

pub open spec fn w1(s: S) -> UInt32;

pub open spec fn IsValidFunctionOrFeatureId(s: S, id: UInt32) -> bool;

pub open spec fn IsImplementedFunctionOrFeature(s: S, id: UInt32) -> bool;

pub open spec fn ResultEqual(result: UInt32, code: UInt32) -> bool;

pub open spec fn PropertiesOf(id: UInt32) -> UInt64;

} // verus!
