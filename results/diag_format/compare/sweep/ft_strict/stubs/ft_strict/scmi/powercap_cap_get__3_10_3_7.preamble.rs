use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Result<Int32, UInt32> = Ok(0i32);
pub const NOT_FOUND: Result<Int32, UInt32> = Err(1u32);
pub const NOT_SUPPORTED: Result<Int32, UInt32> = Err(2u32);

pub open spec fn IsValidPowerCapDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn IsValidCpli(s: S, domain_id: UInt32, cpli: UInt32) -> bool;

pub open spec fn IsPowerCapGetSupported(s: S, domain_id: UInt32, cpli: UInt32) -> bool;

pub open spec fn ResultEqual(a: Result<Int32, UInt32>, b: Result<Int32, UInt32>) -> bool;

pub open spec fn CurrentEnforcedPowerCap(s: S, domain_id: UInt32, cpli: UInt32) -> UInt32;

pub open spec fn PowerCappingDisabled(s: S, domain_id: UInt32) -> bool;

} // verus!
