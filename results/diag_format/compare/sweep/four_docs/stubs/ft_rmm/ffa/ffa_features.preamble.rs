use vstd::prelude::*;
verus! {

pub type Bits1 = u8;
pub type Bits23 = u32;
pub type UInt8 = u8;
pub type UInt32 = u32;
pub type UInt64 = u64;

pub struct S {
    pub dummy: u64,
}

pub const FFA_SUCCESS: UInt32 = 0x84000061;
pub const NOT_SUPPORTED: UInt32 = 0xFFFFFFFF;

pub open spec fn IsValidFunctionOrFeatureId(s: S, id_type: Bits1, function_id: UInt32, feature_reserved: Bits23, feature_id: UInt8) -> bool;

pub open spec fn IsImplementedFunctionOrFeature(s: S, id_type: Bits1, function_id: UInt32, feature_reserved: Bits23, feature_id: UInt8) -> bool;

pub open spec fn ResultEqual(result: UInt32, code: UInt32) -> bool;

pub open spec fn PropertiesOf(s: S, id_type: Bits1, function_id: UInt32, feature_reserved: Bits23, feature_id: UInt8) -> UInt32;

} // verus!
