use vstd::prelude::*;

verus! {

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

pub type RmiStatusCode = u64;

pub const RMI_ERROR_REALM: RmiStatusCode = 1;

pub struct PerformanceLimits {
    pub max: u32,
    pub min: u32,
}

pub struct PerformanceDomainInfo {
    pub limits: PerformanceLimits,
}

pub struct S {
    pub dummy: u64,
}

pub open spec fn IsValidPerformanceDomain(domain_id: u32) -> bool;

pub open spec fn ResultEqual(result: Result<(), RmiStatusCode>, code: RmiStatusCode) -> bool;

pub open spec fn PerformanceDomain(domain_id: u32) -> PerformanceDomainInfo;

} // verus!
