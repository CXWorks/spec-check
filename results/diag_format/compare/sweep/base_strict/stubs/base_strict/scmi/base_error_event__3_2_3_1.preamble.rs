use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type UInt64 = u64;

pub type RmiStatusCode = u64;

pub const RMI_SUCCESS: RmiStatusCode = 0;

pub const RMI_ERROR_INPUT: RmiStatusCode = 1;

pub struct S {
    pub agent_id: UInt32,
    pub protocol_id: UInt32,
    pub error_status: UInt64,
    pub command_count: UInt32,
    pub command_list: Seq<UInt64>,
    pub initial_boot: bool,
}

impl S {
    pub open spec fn agent_id_is_valid(self) -> bool;

    pub open spec fn protocol_id_is_valid(self) -> bool;

    pub open spec fn command_list_is_valid(self) -> bool;

    pub open spec fn is_initial_boot(self) -> bool;
}

pub open spec fn ResultEqual(result: Result<(), RmiStatusCode>, code: RmiStatusCode) -> bool;

pub open spec fn IsRegisteredForBaseErrorNotifications(agent_id: UInt32) -> bool;

pub open spec fn BaseErrorNotificationsEnabled(agent_id: UInt32) -> bool;

pub open spec fn Bits64(value: UInt64, hi: int, lo: int) -> UInt32;

pub open spec fn PlatformCanProcessCommands(s: S) -> bool;

pub open spec fn PlatformIsOperational(s: S) -> bool;

pub open spec fn SomeCommandsFailed(s: S) -> bool;

pub open spec fn IsFailedCommandEntry(command_list: Seq<UInt64>, i: UInt32) -> bool;

} // verus!
