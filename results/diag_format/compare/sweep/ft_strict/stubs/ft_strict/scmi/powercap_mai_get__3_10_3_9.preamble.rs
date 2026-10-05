use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Result<Int32, int> = Ok(0i32);
pub spec const NOT_FOUND: Result<Int32, int> = Err(1int);
pub spec const NOT_SUPPORTED: Result<Int32, int> = Err(2int);

pub open spec fn IsValidPowerCapDomain(s: S, domain_id: UInt32) -> bool;
pub open spec fn IsPowerCapMaiGetSupported(s: S, domain_id: UInt32) -> bool;
pub open spec fn ResultEqual(a: Result<Int32, int>, b: Result<Int32, int>) -> bool;
pub open spec fn EnforcedPowerCapMai(s: S, domain_id: UInt32) -> UInt32;

} // verus!
