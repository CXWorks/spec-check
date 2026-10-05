use vstd::prelude::*;

verus! {

pub type Int64 = i64;
pub type EventNum = u32;
pub type Pe = u64;
pub type IntId = u32;
pub type HandlerState = u8;
pub type IntGroup = u16;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int64 = 0;
pub const NOT_SUPPORTED: Int64 = -1;
pub const INVALID_PARAMETERS: Int64 = -2;
pub const DENIED: Int64 = -3;

pub const HANDLER_UNREGISTERED: HandlerState = 0;

pub const GROUP1_NON_SECURE: IntGroup = 2;

#[allow(non_upper_case_globals)]
pub spec const event: EventNum = 0;

pub open spec fn ResultEqual(result: Int64, code: Int64) -> bool;

pub open spec fn IsSdeiSupported() -> bool;

pub open spec fn IsValidEventNumber(s: S, ev: EventNum) -> bool;

pub open spec fn IsBoundInterruptEvent(s: S, ev: EventNum) -> bool;

pub open spec fn IsPrivateEvent(ev: EventNum) -> bool;

pub open spec fn IsSharedEvent(ev: EventNum) -> bool;

pub open spec fn IsRegisteredPe(pe: Pe) -> bool;

pub open spec fn EventHandlerState(ev: EventNum, pe: Pe) -> HandlerState;

pub open spec fn SharedEventHandlerState(ev: EventNum) -> HandlerState;

pub open spec fn BindSlotReturnedToPool(ev: EventNum) -> bool;

pub open spec fn IsUnregisterPending(s: S, ev: EventNum, pe: Pe) -> bool;

pub open spec fn ReleasedInterrupt(ev: EventNum) -> IntId;

pub open spec fn InterruptConfigRestoredFromBind(s: S, intid: IntId) -> bool;

pub open spec fn IsPhysicalSdeiInstance() -> bool;

pub open spec fn SystemUsesGic() -> bool;

pub open spec fn InterruptGroup(s: S, intid: IntId) -> IntGroup;

pub open spec fn IsInterruptEnabledAtController(s: S, intid: IntId) -> bool;

} // verus!
