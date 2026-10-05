use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub system_power_state: UInt32,
}

pub open spec fn IsSuccessStatus(status: Int32) -> bool;

pub open spec fn CurrentSystemPowerState() -> UInt32;

} // verus!
