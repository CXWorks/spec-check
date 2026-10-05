use vstd::prelude::*;

verus! {

pub struct S {
    pub max_outstanding_async_commands: int,
    pub num_sensors_present: int,
    pub sensor_shared_memory_implemented: bool,
    pub sensor_shared_memory_base: int,
    pub sensor_shared_memory_length: int,
}

pub open spec fn MaxOutstandingAsyncCommandsSupported(s: S) -> int;

pub open spec fn NumSensorsPresent(s: S) -> int;

pub open spec fn SensorSharedMemoryImplemented(s: S) -> bool;

pub open spec fn SensorSharedMemoryBase(s: S) -> int;

pub open spec fn SensorSharedMemoryLength(s: S) -> int;

pub open spec fn AddrInAgentMemoryMap(s: S, addr: int) -> bool;

} // verus!
