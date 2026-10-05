use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type AgentId = nat;

pub struct S {
    pub dummy: nat,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const result: Int32 = -3;

pub spec const caller: AgentId = 0;

pub open spec fn SystemPowerStateNotifySupported(s: S, agent: AgentId) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn Bits(value: UInt32, high: int, low: int) -> int;

pub open spec fn IsPermissibleNotifyEnable(s: S, agent: AgentId, notify_enable: UInt32) -> bool;

pub open spec fn SystemPowerStateNotifyEnabled(s: S, agent: AgentId) -> bool;

} // verus!
