use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type Address = u64;

pub type PsciReturnCode = i32;

pub const SUCCESS: PsciReturnCode = 0;

pub const INVALID_ADDRESS: PsciReturnCode = -9;

pub struct S {
    pub dummy: u64,
}

pub open spec fn IsKnownUnavailableToCaller(addr: Address) -> bool;

} // verus!
