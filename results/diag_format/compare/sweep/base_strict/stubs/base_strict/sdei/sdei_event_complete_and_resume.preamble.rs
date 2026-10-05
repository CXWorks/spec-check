use vstd::prelude::*;

verus! {

pub type Int64 = i64;
pub type UInt64 = u64;
pub type Address = u64;
pub type PeId = u64;
pub type EventId = u32;
pub type BoolWord = u32;

pub struct S {
    pub resume_addr: Address,
}

pub const NOT_SUPPORTED: Int64 = -1;
pub const INVALID_PARAMETERS: Int64 = -2;
pub const DENIED: Int64 = -3;

pub const FALSE: BoolWord = 0;

pub open spec fn SdeiIsSupported() -> bool;

pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;

pub open spec fn DispatcherDetectsInvalidResumeAddress(addr: Address) -> bool;

pub open spec fn CallingPe() -> PeId;

pub open spec fn HandlerRunning(s: S, pe: PeId) -> BoolWord;

pub open spec fn ReturnsToCaller() -> bool;

pub open spec fn RunningEvent(s: S, pe: PeId) -> EventId;

pub open spec fn IsPrivateEvent(s: S, ev: EventId) -> bool;

pub open spec fn IsSharedEvent(s: S, ev: EventId) -> bool;

pub open spec fn EventHandlingCompleteForPe(s: S, ev: EventId, pe: PeId) -> bool;

pub open spec fn EventHandlingCompleteGlobally(s: S, ev: EventId) -> bool;

pub open spec fn PeResumesAtElc(s: S, pe: PeId, addr: Address) -> bool;

pub open spec fn EventTakenPc(s: S, pe: PeId) -> Address;

pub open spec fn ResumeContextMimicsSyncExceptionToElc(s: S, pe: PeId, pc: Address) -> bool;

pub open spec fn InterruptedYieldingSmc(s: S, pe: PeId) -> bool;

pub open spec fn ResumeHandlerRunsBeforeSecureExecution(s: S, pe: PeId) -> bool;

pub open spec fn EventHandlingState(s: S, ev: EventId) -> int;

pub open spec fn ExecutionContext(s: S, pe: PeId) -> int;

} // verus!
