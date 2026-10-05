use vstd::prelude::*;
verus! {

pub type UInt64 = u64;
pub type EventId = u32;
pub type Hart = u64;
pub type SbiReturnCode = i64;
pub type EventStateT = u8;
pub type AttrId = u32;

pub struct S {
    pub dummy: u64,
}

pub spec const SBI_SUCCESS: SbiReturnCode = 0;
pub spec const SBI_ERR_NOT_SUPPORTED: SbiReturnCode = (-2int) as i64;
pub spec const SBI_ERR_INVALID_PARAM: SbiReturnCode = (-3int) as i64;
pub spec const SBI_ERR_INVALID_STATE: SbiReturnCode = (-10int) as i64;

pub spec const UNUSED: EventStateT = 0;
pub spec const REGISTERED: EventStateT = 1;

pub spec const ENTRY_PC: AttrId = 3;
pub spec const ENTRY_ARG: AttrId = 4;

pub spec const event_id: EventId = 0;
pub spec const handler_entry_pc: UInt64 = 0;
pub spec const handler_entry_arg: UInt64 = 0;

pub open spec fn IsValidEventId(id: EventId) -> bool;
pub open spec fn IsReservedEventId(id: EventId) -> bool;
pub open spec fn PlatformSupportsEvent(id: EventId) -> bool;
pub open spec fn EventState(id: EventId) -> EventStateT;
pub open spec fn EventStateForHart(id: EventId, h: Hart) -> EventStateT;
pub open spec fn CallingHart() -> Hart;
pub open spec fn IsLocalEvent(id: EventId) -> bool;
pub open spec fn IsGlobalEvent(id: EventId) -> bool;
pub open spec fn EventAttribute(id: EventId, attr: AttrId) -> UInt64;
pub open spec fn ResultEqual(result: SbiReturnCode, code: SbiReturnCode) -> bool;

} // verus!
