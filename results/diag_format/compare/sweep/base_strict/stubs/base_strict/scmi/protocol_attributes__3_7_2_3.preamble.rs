use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub type UInt64 = u64;

pub type RsiCommandReturnCode = u64;

pub spec const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub spec const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;
pub spec const RSI_ERROR_STATE: RsiCommandReturnCode = 2;
pub spec const RSI_INCOMPLETE: RsiCommandReturnCode = 3;

pub struct S {
    pub dummy: int,
}

pub open spec fn Bits64(x: UInt32, hi: int, lo: int) -> int;

pub open spec fn PlatformMaxOutstandingAsyncCommands() -> int;

pub open spec fn NumSensorsPresent() -> int;

pub open spec fn PlatformImplementsSensorSharedMemory() -> bool;

pub open spec fn SensorSharedMemoryLength() -> UInt32;

pub open spec fn SensorSharedMemoryBase() -> int;

pub open spec fn IsInCallingAgentMemoryMap(addr: int) -> bool;

} // verus!
