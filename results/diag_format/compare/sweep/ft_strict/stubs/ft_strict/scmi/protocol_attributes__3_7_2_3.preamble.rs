use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type RsiCommandReturnCode = u64;

pub spec const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub spec const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;
pub spec const RSI_ERROR_STATE: RsiCommandReturnCode = 2;

pub struct S {
    pub dummy: int,
}

pub open spec fn Bits(value: UInt32, high: int, low: int) -> int;

pub open spec fn PlatformMaxOutstandingAsyncCommands() -> int;

pub open spec fn NumSensorsPresent() -> int;

pub open spec fn PlatformImplementsSensorSharedMemory() -> bool;

pub open spec fn SensorSharedMemoryLength() -> int;

pub open spec fn SensorSharedMemoryBase() -> int;

pub open spec fn IsInCallingAgentMemoryMap(addr: int) -> bool;

} // verus!
