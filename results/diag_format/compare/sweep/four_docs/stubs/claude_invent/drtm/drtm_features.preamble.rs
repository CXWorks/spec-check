use vstd::prelude::*;

verus! {

pub struct S {
    pub drtm_implemented: bool,
    pub normal_world_dce: bool,
    pub boot_pe_id: u64,
}

pub const NOT_SUPPORTED: i64 = -1i64;

pub open spec fn DrtmFunctionIdImplemented(s: S, function_id: u32) -> bool;

pub open spec fn DrtmFeatureIdImplemented(s: S, feature_id: u8) -> bool;

pub open spec fn DrtmUsesNormalWorldDce(s: S) -> bool;

pub open spec fn DrtmBootPeId(s: S) -> u64;

} // verus!
