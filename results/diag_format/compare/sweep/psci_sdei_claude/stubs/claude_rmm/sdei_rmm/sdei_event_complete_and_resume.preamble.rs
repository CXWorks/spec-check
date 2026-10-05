use vstd::prelude::*;

verus! {

pub type Int64 = i64;
pub type Address = u64;
pub type PeId = u64;
pub type EventId = u32;

pub struct S {
    pub dummy: int,
}

pub const NOT_SUPPORTED: Int64 = -1;
pub const INVALID_PARAMETERS: Int64 = -2;
pub const DENIED: Int64 = -3;

pub open spec fn IsSdeiSupported(s: S) -> bool;
pub open spec fn ResultEqual(result: Int64, code: Int64) -> bool;
pub open spec fn IsIdentifiablyInvalidResumeAddress(s: S, addr: Address) -> bool;
pub open spec fn HandlerRunning(s: S, pe: PeId) -> bool;
pub open spec fn CallingPe(s: S) -> PeId;
pub open spec fn ResumeContextExceptionReturnAddress(s: S) -> Address;
pub open spec fn InterruptedPc(s: S) -> Address;
pub open spec fn CurrentEvent(s: S) -> EventId;
pub open spec fn IsPrivateEvent(s: S, event: EventId) -> bool;
pub open spec fn IsSharedEvent(s: S, event: EventId) -> bool;
pub open spec fn EventHandlingComplete(s: S, event: EventId, pe: PeId) -> bool;
pub open spec fn EventHandlingCompleteGlobally(s: S, event: EventId) -> bool;

} // verus!
