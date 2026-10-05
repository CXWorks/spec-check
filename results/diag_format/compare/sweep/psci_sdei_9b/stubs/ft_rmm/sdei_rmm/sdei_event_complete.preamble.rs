use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type SdeiStatusCode = i64;
pub type PeId = u64;
pub type EventNum = u32;

pub struct S {
    pub dummy: u64,
}

pub open spec const NOT_SUPPORTED: SdeiStatusCode = (-1int) as i64;
pub open spec const DENIED: SdeiStatusCode = (-3int) as i64;
pub open spec const SDEI_SUCCESS: Result<(), SdeiStatusCode> = Ok(());

pub open spec const event: EventNum = 0;

pub uninterp spec fn SdeiIsSupported(s: S) -> bool;
pub uninterp spec fn ResultEqual(r: Result<(), SdeiStatusCode>, code: SdeiStatusCode) -> bool;
pub uninterp spec fn HandlerRunning(s: S, pe: PeId) -> bool;
pub uninterp spec fn CallingPe(s: S) -> PeId;
pub uninterp spec fn IsPrivateEvent(s: S, e: EventNum) -> bool;
pub uninterp spec fn IsSharedEvent(s: S, e: EventNum) -> bool;
pub uninterp spec fn EventHandlingComplete(s: S, e: EventNum, pe: PeId) -> bool;
pub uninterp spec fn EventHandlingCompleteGlobally(s: S, e: EventNum) -> bool;

} // verus!
