use vstd::prelude::*;

verus! {

// Note: these MCP servers need authorization before their tools can be used: Airtable,
// Asana, Atlassian, Box, Canva, Clay, Figma, Greenhouse MCP, HubSpot, Intercom, Pylon,
// Slack, Vercel and monday.com. Claude.ai connectors are authorized in claude.ai connector
// settings; other servers through `claude mcp` or /mcp in an interactive session.

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt16 = u16;
pub type SensorId = u16;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_FOUND: Int32 = 1;

#[allow(non_upper_case_globals)]
pub spec const sensor_id: SensorId = 0;

pub open spec fn SensorExists(s: S, id: SensorId) -> bool;
pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;
pub open spec fn SensorUpdateIntervalSupported(s: S, id: SensorId) -> bool;
pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt32;
pub open spec fn SensorUpdateIntervalSec(s: S, id: SensorId) -> UInt32;
pub open spec fn SensorUpdateIntervalExponent(s: S, id: SensorId) -> UInt32;
pub open spec fn SensorIsTimestamped(s: S, id: SensorId) -> bool;
pub open spec fn SensorIsEnabled(s: S, id: SensorId) -> bool;

} // verus!
