use vstd::prelude::*;
verus! {

pub struct S {
    pub sdei_enabled: bool,
    pub bound_events: Set<u32>,
    pub registered_events: Set<u32>,
}

pub const SDEI_SUCCESS: i32 = 0;
pub const SDEI_NOT_SUPPORTED: i32 = -1;
pub const SDEI_INVALID_PARAMETERS: i32 = -2;
pub const SDEI_DENIED: i32 = -3;

#[allow(non_upper_case_globals)]
pub const event: u32 = 0;

pub open spec fn event_is_bound(s: S, ev: u32) -> bool;

pub open spec fn event_handler_is_unregistered(s: S, ev: u32) -> bool;

pub open spec fn event_is_not_bound(s: S, ev: u32) -> bool;

pub open spec fn sdei_supported(s: S) -> bool;

pub open spec fn event_is_valid(s: S, ev: u32) -> bool;

} // verus!
