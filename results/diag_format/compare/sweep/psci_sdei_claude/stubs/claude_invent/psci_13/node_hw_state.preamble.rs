use vstd::prelude::*;

verus! {

pub type Bits64 = u64;
pub type Bits32 = u32;

pub struct S {
    pub dummy: int,
}

pub const NOT_SUPPORTED: Bits64 = 0xFFFF_FFFF_FFFF_FFFFu64;
pub const INVALID_PARAMETERS: Bits64 = 0xFFFF_FFFF_FFFF_FFFEu64;
pub const HW_ON: Bits64 = 0u64;
pub const HW_OFF: Bits64 = 1u64;
pub const HW_STANDBY: Bits64 = 2u64;

pub open spec fn PsciNodeHwStateImplemented(s: S) -> bool;
pub open spec fn IsValidPowerNode(s: S, target_cpu: Bits64, power_level: Bits32) -> bool;
pub open spec fn NodeHwInRunState(s: S, target_cpu: Bits64, power_level: Bits32) -> bool;
pub open spec fn NodeHwInPowerdownState(s: S, target_cpu: Bits64, power_level: Bits32) -> bool;
pub open spec fn NodeHwInStandbyState(s: S, target_cpu: Bits64, power_level: Bits32) -> bool;

} // verus!
