use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type Event = i32;
pub type Pe = i32;
pub type InterruptId = u32;
pub type HandlerState = u32;
pub type InterruptGroupType = u32;

pub trait PickErr {
    type Out;
}

impl<E> PickErr for ((), E) {
    type Out = E;
}

pub type Result<T, E> = <(T, E) as PickErr>::Out;

pub enum SdeiCommandReturnCode {
    SUCCESS,
    NOT_SUPPORTED,
    INVALID_PARAMETERS,
    DENIED,
    PENDING,
    OUT_OF_RESOURCE,
}

pub struct S {
    pub sdei_supported: bool,
    pub physical_instance: bool,
    pub uses_gic: bool,
}

pub spec const SUCCESS: SdeiCommandReturnCode = SdeiCommandReturnCode::SUCCESS;
pub spec const NOT_SUPPORTED: SdeiCommandReturnCode = SdeiCommandReturnCode::NOT_SUPPORTED;
pub spec const INVALID_PARAMETERS: SdeiCommandReturnCode = SdeiCommandReturnCode::INVALID_PARAMETERS;
pub spec const DENIED: SdeiCommandReturnCode = SdeiCommandReturnCode::DENIED;

pub spec const HANDLER_UNREGISTERED: HandlerState = 0;
pub spec const HANDLER_REGISTERED: HandlerState = 1;

pub spec const GROUP1_NON_SECURE: InterruptGroupType = 2;
pub spec const GROUP0: InterruptGroupType = 0;
pub spec const GROUP1_SECURE: InterruptGroupType = 1;

pub open spec fn ResultEqual(result: SdeiCommandReturnCode, code: SdeiCommandReturnCode) -> bool;
pub open spec fn IsSdeiSupported(s: S) -> bool;
pub open spec fn IsValidEventNumber(s: S, event: Int32) -> bool;
pub open spec fn IsBoundInterruptEvent(s: S, event: Int32) -> bool;
pub open spec fn IsPrivateEvent(s: S, event: Int32) -> bool;
pub open spec fn IsSharedEvent(s: S, event: Int32) -> bool;
pub open spec fn IsRegisteredPe(s: S, pe: Pe) -> bool;
pub open spec fn EventHandlerState(s: S, event: Int32, pe: Pe) -> HandlerState;
pub open spec fn SharedEventHandlerState(s: S, event: Int32) -> HandlerState;
pub open spec fn BindSlotReturnedToPool(s: S, event: Int32) -> bool;
pub open spec fn IsUnregisterPending(s: S, event: Int32, pe: Pe) -> bool;
pub open spec fn ReleasedInterrupt(s: S, event: Int32) -> InterruptId;
pub open spec fn InterruptConfigRestoredFromBind(s: S, intr: InterruptId) -> bool;
pub open spec fn IsPhysicalSdeiInstance(s: S) -> bool;
pub open spec fn SystemUsesGic(s: S) -> bool;
pub open spec fn InterruptGroup(s: S, intr: InterruptId) -> InterruptGroupType;
pub open spec fn IsInterruptEnabledAtController(s: S, intr: InterruptId) -> bool;

} // verus!
