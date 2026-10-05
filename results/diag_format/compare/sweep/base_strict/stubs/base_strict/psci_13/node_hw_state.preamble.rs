use vstd::prelude::*;

verus! {

pub type Int64 = i64;
pub type UInt64 = u64;
pub type UInt32 = u32;

pub struct S {
    pub node_power_states: Map<(UInt64, UInt32), PowerState>,
    pub ghost_counter: nat,
}

#[derive(PartialEq, Eq, Structural)]
pub enum PowerState {
    Run,
    Powerdown,
    RetentionStandby,
}

pub spec const RUN: PowerState = PowerState::Run;
pub spec const POWERDOWN: PowerState = PowerState::Powerdown;
pub spec const RETENTION_STANDBY: PowerState = PowerState::RetentionStandby;

pub spec const NOT_SUPPORTED: Int64 = (-1int) as i64;
pub spec const INVALID_PARAMETERS: Int64 = (-2int) as i64;
pub spec const HW_ON: Int64 = 0;
pub spec const HW_OFF: Int64 = 1;
pub spec const HW_STANDBY: Int64 = 2;

pub spec const target_cpu: UInt64 = 0x100;
pub spec const power_level: UInt32 = 0;

pub open spec fn IsNodeHwStateImplemented() -> bool;

pub open spec fn IsValidNode(s: S, cpu: UInt64, level: UInt32) -> bool;

pub open spec fn PowerControllerViewOfNode(s: S, cpu: UInt64, level: UInt32) -> PowerState;

pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;

} // verus!
