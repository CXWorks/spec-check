use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub type RsiCommandReturnCode = u64;

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;

pub struct S {
    pub dummy: int,
}

pub open spec fn BitsOf(x: UInt32, lo: int, hi: int) -> UInt32;

pub trait BitSliceIndex {
    spec fn spec_index(self, r: core::ops::Range<int>) -> UInt32;
}

impl BitSliceIndex for u32 {
    open spec fn spec_index(self, r: core::ops::Range<int>) -> UInt32 {
        BitsOf(self, r.start, r.end)
    }
}

pub open spec fn PlatformMaxOutstandingAsyncCommands() -> UInt32;

pub open spec fn NumSensorsPresent() -> UInt32;

pub open spec fn PlatformImplementsSensorSharedMemory() -> bool;

pub open spec fn SensorSharedMemoryLength() -> UInt32;

pub open spec fn SensorSharedMemoryBase() -> UInt32;

pub open spec fn IsInCallerMemoryMap(addr: UInt32) -> bool;

pub open spec fn IsValid(x: UInt32) -> bool;

} // verus!
