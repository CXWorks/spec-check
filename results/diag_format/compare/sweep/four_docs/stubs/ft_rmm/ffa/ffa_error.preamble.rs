use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt16 = u16;
pub type Conduit = u8;

pub const SMC: Conduit = 0;
pub const HVC: Conduit = 1;

pub struct S {
    pub dummy: int,
}

pub open spec fn ErrorCodeReturnedToPreviousCaller(s: S, error_code: Int32) -> bool;

pub open spec fn IsNonSecureVirtualInstance(s: S) -> bool;

pub open spec fn ConduitIs(s: S, c: Conduit) -> bool;

pub open spec fn ErrorCodeDeliveredTo(s: S, target_id: UInt16, target_vcpu: UInt16, error_code: Int32) -> bool;

} // verus!
