use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = spec_fn(S) -> int;

pub struct S {
    pub domain_id: u32,
    pub cpli: u32,
    pub power_cap: u32,
}

pub const SUCCESS: int32 = 0;
pub const NOT_FOUND: int32 = -1;
pub const NOT_SUPPORTED: int32 = -2;

pub open spec fn domain_id(s: S) -> int;

pub open spec fn cpli(s: S) -> int;

pub open spec fn power_cap(s: S) -> int;

} // verus!
