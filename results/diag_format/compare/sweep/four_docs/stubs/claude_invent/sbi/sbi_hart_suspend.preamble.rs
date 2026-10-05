use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;

pub struct S {
    pub hart_id: u64,
    pub regs: Seq<u64>,
    pub csrs: Seq<u64>,
}

pub open spec fn IsRetentiveSuspendType(suspend_type: UInt32) -> bool;

pub open spec fn SbiRetIsSuccess(ret_error: i64) -> bool;

pub open spec fn HartRegistersAndCsrsPreserved(old_s: S, new_s: S) -> bool;

} // verus!
