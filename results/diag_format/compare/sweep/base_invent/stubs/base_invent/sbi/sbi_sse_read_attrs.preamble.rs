use vstd::prelude::*;
verus! {

pub type sbiret = i64;

pub const SBI_SUCCESS: sbiret = 0;
pub const SBI_ERR_FAILED: sbiret = -1;
pub const SBI_ERR_NOT_SUPPORTED: sbiret = -2;
pub const SBI_ERR_INVALID_PARAM: sbiret = -3;
pub const SBI_ERR_INVALID_ADDRESS: sbiret = -5;
pub const SBI_ERR_BAD_RANGE: sbiret = -11;

pub const XLEN: u64 = 64;

pub struct S {
    pub dummy: u64,
}

pub open spec fn event_id(s: S) -> u32;

pub open spec fn attr_count(s: S) -> u32;

pub open spec fn base_attr_id(s: S) -> u32;

pub open spec fn is_reserved_event_attr_id(s: S, attr_id: int) -> bool;

pub open spec fn output_phys_lo(s: S) -> u64;

pub open spec fn is_aligned(addr: u64, align: u64) -> bool;

pub open spec fn platform_supports_event_id(s: S, id: u32) -> bool;

} // verus!
