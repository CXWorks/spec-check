use vstd::prelude::*;
verus! {

pub type Address = u64;
pub type Int64 = i64;
pub type PeId = u64;
pub type EventNum = u32;

pub struct S {
    pub sdei_supported: bool,
    pub calling_pe: PeId,
    pub current_event: EventNum,
    pub resume_context_elr: Address,
    pub interrupted_pc: Address,
}

pub spec const SDEI_SUCCESS: Int64 = 0;
pub spec const NOT_SUPPORTED: Int64 = (-1) as i64;
pub spec const INVALID_PARAMETERS: Int64 = (-2) as i64;
pub spec const DENIED: Int64 = (-3) as i64;

pub uninterp spec fn IsSdeiSupported(s: S) -> bool;
pub uninterp spec fn ResultEqual(a: Int64, b: Int64) -> bool;
pub uninterp spec fn IsIdentifiablyInvalidResumeAddress(s: S, addr: Address) -> bool;
pub uninterp spec fn CallingPe(s: S) -> PeId;
pub uninterp spec fn HandlerRunning(s: S, pe: PeId) -> bool;
pub uninterp spec fn ResumeContextExceptionReturnAddress(s: S) -> Address;
pub uninterp spec fn InterruptedPc(s: S) -> Address;
pub uninterp spec fn CurrentEvent(s: S) -> EventNum;
pub uninterp spec fn IsPrivateEvent(s: S, ev: EventNum) -> bool;
pub uninterp spec fn IsSharedEvent(s: S, ev: EventNum) -> bool;
pub uninterp spec fn EventHandlingComplete(s: S, ev: EventNum, pe: PeId) -> bool;
pub uninterp spec fn EventHandlingCompleteGlobally(s: S, ev: EventNum) -> bool;

} // verus!
