use vstd::prelude::*;

verus! {

pub type Address = u64;

pub type SdeiStatusCode = i64;

pub type PeId = u64;

pub type EventId = u32;

pub struct S {
    pub dummy: int,
}

pub spec const NOT_SUPPORTED: SdeiStatusCode = (-1int) as i64;

pub spec const INVALID_PARAMETERS: SdeiStatusCode = (-2int) as i64;

pub spec const DENIED: SdeiStatusCode = (-3int) as i64;

pub spec const SDEI_SUCCESS: Result<(), SdeiStatusCode> = Ok(());

pub open spec fn SdeiIsSupported(s: S) -> bool;

pub open spec fn ResultEqual(result: Result<(), SdeiStatusCode>, code: SdeiStatusCode) -> bool;

pub open spec fn DispatcherDetectsInvalidResumeAddress(s: S, resume_addr: Address) -> bool;

pub open spec fn HandlerRunning(s: S, pe: PeId) -> bool;

pub open spec fn CallingPe() -> PeId;

pub open spec fn ReturnsToCaller(s: S) -> bool;

pub open spec fn IsPrivateEvent(s: S, event: EventId) -> bool;

pub open spec fn IsSharedEvent(s: S, event: EventId) -> bool;

pub open spec fn RunningEvent(s: S, pe: PeId) -> EventId;

pub open spec fn EventHandlingCompleteForPe(s: S, event: EventId, pe: PeId) -> bool;

pub open spec fn EventHandlingCompleteGlobally(s: S, event: EventId) -> bool;

pub open spec fn PeResumesAtElc(s: S, pe: PeId, addr: Address) -> bool;

pub open spec fn ResumeContextMimicsSyncExceptionToElc(s: S, pe: PeId, pc: Address) -> bool;

pub open spec fn EventTakenPc(s: S, pe: PeId) -> Address;

pub open spec fn InterruptedYieldingSmc(s: S, pe: PeId) -> bool;

pub open spec fn ResumeHandlerRunsBeforeSecureExecution(s: S, pe: PeId) -> bool;

} // verus!
