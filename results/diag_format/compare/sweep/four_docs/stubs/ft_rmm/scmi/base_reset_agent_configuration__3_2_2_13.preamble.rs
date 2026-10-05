use vstd::prelude::*;

verus! {

pub struct UInt32 {
    pub v: u32,
}

impl UInt32 {
    pub open spec fn spec_index(self, i: int) -> int;
}

pub type RmiStatusCode = u32;

pub type CommandId = u32;

pub type CallerId = u32;

pub struct Permissions {
    pub bits: u64,
}

pub struct Resources {
    pub id: u64,
}

pub struct S {
    pub dummy: u64,
}

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Result<T, E> {
    pub open spec fn is_Ok(self) -> bool;
    pub open spec fn is_Err(self) -> bool;
}

pub const SUCCESS: RmiStatusCode = 0;
pub const NOT_FOUND: RmiStatusCode = 1;
pub const INVALID_PARAMETERS: RmiStatusCode = 2;
pub const NOT_SUPPORTED: RmiStatusCode = 3;
pub const DENIED: RmiStatusCode = 4;

pub const BASE_RESET_AGENT_CONFIGURATION: CommandId = 0x0B;

pub spec const caller: CallerId = 0;

pub open spec fn AgentExists(s: S, agent_id: UInt32) -> bool;

pub open spec fn AreValidResetFlags(s: S, flags: UInt32) -> bool;

pub open spec fn IsCommandImplemented(s: S, cmd: CommandId) -> bool;

pub open spec fn CallerMayResetAgent(s: S, caller: CallerId, agent_id: UInt32) -> bool;

pub open spec fn ResultEqual(result: Result<(), RmiStatusCode>, code: RmiStatusCode) -> bool;

pub open spec fn ResourcesDedicatedTo(s: S, agent_id: UInt32) -> Resources;

pub open spec fn SharedResourcesUsedBy(s: S, agent_id: UInt32) -> Resources;

pub open spec fn AccessPermissions(s: S, agent_id: UInt32) -> Permissions;

pub open spec fn ImplDefinedDefaultPermissions(s: S, agent_id: UInt32) -> Permissions;

} // verus!
