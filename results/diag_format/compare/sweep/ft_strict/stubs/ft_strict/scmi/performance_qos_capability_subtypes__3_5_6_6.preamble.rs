use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type RmiStatusCode = u64;

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Result<T, E> {
    pub open spec fn is_Ok(&self) -> bool {
        match self {
            Result::Ok(_) => true,
            Result::Err(_) => false,
        }
    }

    pub open spec fn is_Err(&self) -> bool {
        match self {
            Result::Ok(_) => false,
            Result::Err(_) => true,
        }
    }
}

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: RmiStatusCode = 0;
pub const NOT_FOUND: RmiStatusCode = 1;
pub const INVALID_PARAMETERS: RmiStatusCode = 2;

pub open spec fn IsValidPerfDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn IsValidQosCapabilityType(s: S, domain_id: UInt32, capability_type: UInt32) -> bool;

pub open spec fn PopCount(x: int) -> int;

pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> int;

pub open spec fn ResultEqual(result: Result<(), RmiStatusCode>, code: RmiStatusCode) -> bool;

pub open spec fn SupportedOemQosSubtypes(s: S, domain_id: UInt32, capability_type: UInt32) -> int;

pub open spec fn SupportedArchitectedQosSubtypes(s: S, domain_id: UInt32, capability_type: UInt32) -> int;

} // verus!
