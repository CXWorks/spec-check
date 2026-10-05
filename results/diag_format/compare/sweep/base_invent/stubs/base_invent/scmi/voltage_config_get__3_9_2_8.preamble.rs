use vstd::prelude::*;

verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: int32 = 0i32;
pub spec const NOT_SUPPORTED: int32 = -1i32;
pub spec const DENIED: int32 = -3i32;
pub spec const NOT_FOUND: int32 = -4i32;

pub spec const domain_id: uint32 = 0u32;

pub uninterp spec fn domain_id_not_valid(old_s: S, domain_id: uint32) -> bool;

pub uninterp spec fn request_not_supported(old_s: S) -> bool;

pub uninterp spec fn agent_not_allowed(old_s: S, domain_id: uint32) -> bool;

pub uninterp spec fn config_bits_3_0_mode_valid(config: uint32) -> bool;

pub uninterp spec fn config_bits_31_4_zero(config: uint32) -> bool;

} // verus!
