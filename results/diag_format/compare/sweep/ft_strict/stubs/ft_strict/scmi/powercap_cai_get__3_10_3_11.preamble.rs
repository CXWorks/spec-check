use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Result<Int32, UInt32> = Ok(0i32);
pub spec const NOT_FOUND: Result<Int32, UInt32> = Err(1u32);
pub spec const NOT_SUPPORTED: Result<Int32, UInt32> = Err(2u32);

pub open spec fn IsValidPowercapDomain(s: S, domain_id: UInt32) -> bool;
pub open spec fn IsValidCpli(s: S, domain_id: UInt32, cpli: UInt32) -> bool;
pub open spec fn DomainSupportsCpc(s: S, domain_id: UInt32) -> bool;
pub open spec fn IsCaiGetSupported(s: S, domain_id: UInt32) -> bool;
pub open spec fn ResultEqual(a: Result<Int32, UInt32>, b: Result<Int32, UInt32>) -> bool;
pub open spec fn CurrentCai(s: S, domain_id: UInt32, cpli: UInt32) -> UInt32;

} // verus!
