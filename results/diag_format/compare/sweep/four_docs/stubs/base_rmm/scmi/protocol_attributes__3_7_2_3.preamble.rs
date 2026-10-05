use vstd::prelude::*;

verus! {

pub type RsiCommandReturnCode = u32;

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;
pub const RSI_ERROR_STATE: RsiCommandReturnCode = 2;
pub const RSI_INCOMPLETE: RsiCommandReturnCode = 3;

pub struct S {
    pub attributes: u32,
    pub sensor_reg_len: u32,
    pub sensor_reg_address_low: u64,
    pub sensor_reg_address_high: u64,
}

pub open spec fn PlatformMaxOutstandingAsyncCommands() -> u32;

pub open spec fn NumSensorsPresent() -> u32;

pub open spec fn PlatformImplementsSensorSharedMemory() -> bool;

pub open spec fn SensorSharedMemoryLength() -> u32;

pub open spec fn SensorSharedMemoryBase() -> u64;

pub open spec fn IsInCallerMemoryMap(addr: u64) -> bool;

pub open spec fn IsValid(v: u64) -> bool;

} // verus!
