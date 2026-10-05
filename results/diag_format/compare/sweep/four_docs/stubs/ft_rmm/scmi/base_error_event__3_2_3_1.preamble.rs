use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub enum BaseErrorEventResult {
    Success,
    Failure,
}

pub open spec fn IsRegisteredForBaseErrorNotification(s: S, agent: UInt32) -> bool;

pub open spec fn PlatformImplementsBaseErrorNotification(s: S) -> bool;

pub open spec fn NotificationSentTo(s: S, agent: UInt32) -> bool;

pub open spec fn IsInitialBoot(s: S, agent: UInt32) -> bool;

pub open spec fn IsFatalError(s: S) -> bool;

pub open spec fn CommandsNotProcessed() -> [UInt32; 1];

} // verus!
