use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type PeId = u64;
pub type IntId = u32;
pub type BindSlot = u64;
pub type SdeiCommandReturnCode = i64;
pub type HandlerStateKind = u8;
pub type InterruptConfiguration = u64;

pub struct S {
    pub sdei_supported: bool,
}

pub spec const SUCCESS: SdeiCommandReturnCode = 0;
pub spec const NOT_SUPPORTED: SdeiCommandReturnCode = -1;
pub spec const INVALID_PARAMETERS: SdeiCommandReturnCode = -2;
pub spec const DENIED: SdeiCommandReturnCode = -3;
pub spec const PENDING: SdeiCommandReturnCode = -5;
pub spec const OUT_OF_RESOURCE: SdeiCommandReturnCode = -10;

pub spec const SDEI_SUCCESS: Result<(), SdeiCommandReturnCode> = Ok(());

pub spec const HANDLER_UNREGISTERED: HandlerStateKind = 0;
pub spec const HANDLER_REGISTERED: HandlerStateKind = 1;
pub spec const HANDLER_ENABLED: HandlerStateKind = 2;
pub spec const HANDLER_RUNNING: HandlerStateKind = 3;

pub open spec fn IsSdeiSupported(s: S) -> bool;
pub open spec fn ResultEqual(result: Result<(), SdeiCommandReturnCode>, code: SdeiCommandReturnCode) -> bool;
pub open spec fn IsValidEvent(s: S, event: Int32) -> bool;
pub open spec fn IsEventBound(s: S, event: Int32) -> bool;
pub open spec fn IsPrivateEvent(s: S, event: Int32) -> bool;
pub open spec fn IsSharedEvent(s: S, event: Int32) -> bool;
pub open spec fn RegisteredPes(s: S, event: Int32) -> Set<PeId>;
pub open spec fn HandlerState(s: S, event: Int32, pe: PeId) -> HandlerStateKind;
pub open spec fn BindSlotOf(s: S, event: Int32) -> BindSlot;
pub open spec fn BoundInterrupt(s: S, event: Int32) -> IntId;
pub open spec fn InterruptConfig(s: S, intid: IntId) -> InterruptConfiguration;
pub open spec fn PreBindInterruptConfig(s: S, intid: IntId) -> InterruptConfiguration;
pub open spec fn IsInterruptEnabled(s: S, intid: IntId) -> bool;

} // verus!
