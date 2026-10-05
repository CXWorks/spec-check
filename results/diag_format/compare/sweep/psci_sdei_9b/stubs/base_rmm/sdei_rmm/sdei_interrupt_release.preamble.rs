use vstd::prelude::*;

verus! {

pub type Int64 = i64;
pub type EventId = u32;
pub type PeId = u64;
pub type InterruptId = u32;
pub type HandlerStateValue = u32;
pub type BindSlot = u32;
pub type InterruptConfigValue = u64;

pub struct S {
    pub sdei_supported: bool,
}

pub spec const event: EventId = 0;

pub spec const SUCCESS: Int64 = 0;
pub spec const NOT_SUPPORTED: Int64 = -1;
pub spec const INVALID_PARAMETERS: Int64 = -2;
pub spec const DENIED: Int64 = -3;

pub spec const HANDLER_UNREGISTERED: HandlerStateValue = 0;

pub open spec fn IsSdeiSupported() -> bool;

pub open spec fn ResultEqual(result: Int64, code: Int64) -> bool;

pub open spec fn IsValidEvent(ev: EventId) -> bool;

pub open spec fn IsEventBound(ev: EventId) -> bool;

pub open spec fn IsPrivateEvent(ev: EventId) -> bool;

pub open spec fn IsSharedEvent(ev: EventId) -> bool;

pub open spec fn RegisteredPes(ev: EventId) -> Set<PeId>;

pub open spec fn HandlerState(ev: EventId, pe: PeId) -> HandlerStateValue;

pub open spec fn BindSlotOf(ev: EventId) -> BindSlot;

pub open spec fn BoundInterrupt(ev: EventId) -> InterruptId;

pub open spec fn InterruptConfig(intr: InterruptId) -> InterruptConfigValue;

pub open spec fn PreBindInterruptConfig(intr: InterruptId) -> InterruptConfigValue;

pub open spec fn IsInterruptEnabled(intr: InterruptId) -> bool;

} // verus!
