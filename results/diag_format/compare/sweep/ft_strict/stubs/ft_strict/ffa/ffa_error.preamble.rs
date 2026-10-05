use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: int,
}

pub open spec fn Bits(value: UInt32, hi: nat, lo: nat) -> UInt32;

pub open spec fn ErrorDeliveredToVcpu(vm_id: UInt32, vcpu_id: UInt32, error_code: Int32) -> bool;

} // verus!
