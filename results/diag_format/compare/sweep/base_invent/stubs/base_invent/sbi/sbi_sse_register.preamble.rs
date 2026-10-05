use vstd::prelude::*;
verus! {

pub type sbiret = i64;

pub const SBI_SUCCESS: sbiret = 0;
pub const SBI_ERR_NOT_SUPPORTED: sbiret = -2;
pub const SBI_ERR_INVALID_PARAM: sbiret = -3;
pub const SBI_ERR_INVALID_STATE: sbiret = -10;

pub const event_id: i64 = 1;
pub const handler_entry_pc: u64 = 2;

pub struct S {
    pub dummy: int,
}

pub open spec fn EventIsUnused(s: S, id: i64) -> bool;

pub open spec fn EventIsRegistered(s: S, id: i64) -> bool;

} // verus!
