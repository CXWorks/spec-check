use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub enum Int64Result {
    Ok(i64),
    Err(i64),
}

impl Int64Result {
    pub open spec fn is_Ok(self) -> bool;
}

pub type int64 = Int64Result;

pub struct S {
    pub sdei_event_context_param_id: UInt32,
    pub sdei_event_context_handler_running: bool,
    pub sdei_event_context_calling_pe: UInt32,
    pub sdei_event_context_event_handler_pe: UInt32,
    pub sdei_event_context_sdei_supported: bool,
}

pub const SDEI_ERROR_NOT_SUPPORTED: i64 = -1;
pub const SDEI_ERROR_INVALID_PARAMETERS: i64 = -2;
pub const SDEI_ERROR_DENIED: i64 = -3;

pub open spec fn ResultEqual(result: int64, code: i64) -> bool;

} // verus!
