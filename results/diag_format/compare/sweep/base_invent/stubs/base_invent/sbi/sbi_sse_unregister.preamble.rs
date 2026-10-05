use vstd::prelude::*;
verus! {

pub type uint32_t = u32;

pub struct sbiret {
    pub error: i64,
    pub event_id: uint32_t,
}

pub struct S {
    pub dummy: int,
}

pub open spec fn event_is_registered(s: S, event_id: uint32_t) -> bool;

pub open spec fn event_state_is_unused(s: S, event_id: uint32_t) -> bool;

pub open spec fn event_state_is_not_registered(old_s: S, event_id: uint32_t, new_s: S) -> bool;

} // verus!
