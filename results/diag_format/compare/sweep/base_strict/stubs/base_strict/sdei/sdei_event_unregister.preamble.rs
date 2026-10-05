use vstd::prelude::*;

verus! {

// Note: these MCP servers need authorization before their tools can be used: claude.ai Airtable, Asana, Atlassian, Box, Canva, Clay, Figma, Greenhouse MCP, HubSpot, Intercom, Pylon, Slack, Vercel and monday.com.
// To authorize a claude.ai connector, use its claude.ai connector settings. For other servers, run `claude mcp` or /mcp in an interactive session.

pub type Int64 = i64;
pub type Int32 = i32;
pub type PeId = u64;

pub struct S {
    pub dummy: int,
}

pub enum HandlerState {
    Idle,
    Running,
    Pending,
}

pub const SUCCESS: Int64 = 0;
pub const NOT_SUPPORTED: Int64 = -1;
pub const INVALID_PARAMETERS: Int64 = -2;
pub const DENIED: Int64 = -3;
pub const PENDING: Int64 = -5;

pub open spec fn IsSdeiSupported() -> bool;

pub open spec fn ResultEqual(result: Int64, code: Int64) -> bool;

pub open spec fn IsValidEventNumber(event: Int32) -> bool;

pub open spec fn IsEventRegisteredByClient(event: Int32) -> bool;

pub open spec fn IsHandlerRunning(event: Int32) -> bool;

pub open spec fn IsUnregisterPending(event: Int32) -> bool;

pub open spec fn IsSharedEvent(event: Int32) -> bool;

pub open spec fn IsEventRegisteredGlobally(event: Int32) -> bool;

pub open spec fn IsPrivateEvent(event: Int32) -> bool;

pub open spec fn IsEventRegisteredOnPe(event: Int32, pe: PeId) -> bool;

pub open spec fn CurrentPe() -> PeId;

pub open spec fn IsEventDeliverableToClient(event: Int32) -> bool;

pub open spec fn EventHandlerState(event: Int32, s: S) -> HandlerState;

} // verus!
