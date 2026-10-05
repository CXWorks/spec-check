use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub enum RsiCommandReturnCode {
    RsiErrorInput,
    RsiErrorState,
    RsiErrorUnknown,
}

pub spec const RSI_SUCCESS: Result<(), RsiCommandReturnCode> = Ok(());

pub struct S {
    pub dummy: u64,
}

pub open spec fn NotificationSentTo(s: S, agent_id: UInt32) -> bool;

pub open spec fn IsRegisteredForBaseErrorNotifications(s: S, agent_id: UInt32) -> bool;

pub open spec fn PlatformImplementsBaseErrorNotifications(s: S) -> bool;

pub open spec fn IsInitialBoot(s: S, agent_id: UInt32) -> bool;

pub open spec fn BaseErrorNotificationsEnabled(s: S, agent_id: UInt32) -> bool;

pub open spec fn Bits(value: UInt32, hi: UInt32, lo: UInt32) -> UInt32;

pub open spec fn PlatformCanProcessCommands(s: S) -> bool;

pub open spec fn PlatformIsOperational(s: S) -> bool;

pub open spec fn SomeCommandsFailed(s: S) -> bool;

pub open spec fn IsFailedCommandEntry(s: S, command_list: [UInt32; 1], i: UInt32) -> bool;

} // verus!
