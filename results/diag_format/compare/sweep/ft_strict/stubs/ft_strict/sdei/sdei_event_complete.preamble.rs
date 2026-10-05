use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type UInt64 = u64;

pub type SdeiStatusCode = i32;

pub type PeId = u64;

pub type EventNum = u32;

pub struct S {
    pub sdei_supported: bool,
    pub calling_pe: PeId,
}

pub spec const NOT_SUPPORTED: SdeiStatusCode = (-1) as SdeiStatusCode;

pub spec const INVALID_PARAMETERS: SdeiStatusCode = (-2) as SdeiStatusCode;

pub spec const DENIED: SdeiStatusCode = (-3) as SdeiStatusCode;

pub spec const PENDING: SdeiStatusCode = (-5) as SdeiStatusCode;

pub spec const OUT_OF_RESOURCE: SdeiStatusCode = (-10) as SdeiStatusCode;

pub spec const SDEI_SUCCESS: Result<(), SdeiStatusCode> = Ok(());

pub uninterp spec fn SdeiIsSupported(s: S) -> bool;

pub uninterp spec fn ResultEqual(result: Result<(), SdeiStatusCode>, code: SdeiStatusCode) -> bool;

pub uninterp spec fn CallingPe(s: S) -> PeId;

pub uninterp spec fn HandlerRunning(s: S, pe: PeId) -> bool;

pub uninterp spec fn CallReturns(s: S, pe: PeId) -> bool;

pub uninterp spec fn ExecutionResumesAtInterruptedContext(s: S, pe: PeId) -> bool;

pub uninterp spec fn RunningEvent(s: S, pe: PeId) -> EventNum;

pub uninterp spec fn IsPrivateEvent(s: S, event: EventNum) -> bool;

pub uninterp spec fn IsSharedEvent(s: S, event: EventNum) -> bool;

pub uninterp spec fn EventHandlingCompleteForPe(s: S, event: EventNum, pe: PeId) -> bool;

pub uninterp spec fn EventHandlingCompleteGlobally(s: S, event: EventNum) -> bool;

} // verus!
