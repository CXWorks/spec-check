use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type AgentId = u32;
pub type CallerId = u32;
pub type CommandId = u32;
pub type ResourceSet = Set<int>;
pub type Permissions = Set<int>;

pub struct S {
    pub agents: Set<AgentId>,
    pub implemented_commands: Set<CommandId>,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const DENIED: Int32 = -3;
pub const NOT_FOUND: Int32 = -4;

pub const BASE_RESET_AGENT_CONFIGURATION: CommandId = 11;

pub spec const agent_id: AgentId = 0;
pub spec const caller: CallerId = 0;
pub spec const flags: Seq<UInt32> = Seq::empty();

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

pub open spec fn AgentExists(s: S, aid: AgentId) -> bool;

pub open spec fn AreValidResetFlags(s: S, fl: Seq<UInt32>) -> bool;

pub open spec fn IsCommandImplemented(s: S, cmd: CommandId) -> bool;

pub open spec fn CallerMayResetAgent(s: S, c: CallerId, aid: AgentId) -> bool;

pub open spec fn ResourcesDedicatedTo(s: S, aid: AgentId) -> ResourceSet;

pub open spec fn SharedResourcesUsedBy(s: S, aid: AgentId) -> ResourceSet;

pub open spec fn AccessPermissions(s: S, aid: AgentId) -> Permissions;

pub open spec fn ImplDefinedDefaultPermissions(s: S, aid: AgentId) -> Permissions;

} // verus!
