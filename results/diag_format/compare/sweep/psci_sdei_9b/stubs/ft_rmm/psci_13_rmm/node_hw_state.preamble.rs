use vstd::prelude::*;
verus! {

pub type MpidrTarget = u64;

pub type PowerLevel = u32;

pub type ReturnCode = i32;

pub type PowerState = u8;

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Result<T, E> {
    pub open spec fn is_Ok(self) -> bool {
        match self {
            Result::Ok(_) => true,
            Result::Err(_) => false,
        }
    }

    pub open spec fn is_Err(self) -> bool {
        match self {
            Result::Ok(_) => false,
            Result::Err(_) => true,
        }
    }
}

pub struct S {
    pub node_hw_state_implemented: bool,
    pub num_cpus: nat,
    pub max_power_level: nat,
}

pub const HW_ON: ReturnCode = 0;
pub const HW_OFF: ReturnCode = 1;
pub const HW_STANDBY: ReturnCode = 2;
pub const NOT_SUPPORTED: ReturnCode = -1;
pub const INVALID_PARAMETERS: ReturnCode = -2;

pub const RUN: PowerState = 0;
pub const POWERDOWN: PowerState = 1;
pub const STANDBY_OR_RETENTION: PowerState = 2;

pub open spec fn NodeHwStateImplemented(s: S) -> bool;

pub open spec fn IsValidNode(s: S, target_cpu: MpidrTarget, power_level: PowerLevel) -> bool;

pub open spec fn PowerControlView(s: S, target_cpu: MpidrTarget, power_level: PowerLevel) -> PowerState;

pub open spec fn ResultEqual(result: Result<(), ReturnCode>, code: ReturnCode) -> bool;

} // verus!
