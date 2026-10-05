use vstd::prelude::*;

verus! {

pub type uint32 = u32;

pub type RmiStatusCode = u32;

pub const SUCCESS: RmiStatusCode = 0;

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

pub open spec fn ResultEqual(result: Result<(), RmiStatusCode>, code: RmiStatusCode) -> bool;

pub open spec fn AllDomainsQosAtPlatformDefault(s: S, capability: uint32) -> bool;

pub open spec fn DomainAndSiblingsQosAtPlatformDefault(s: S, domain_id: uint32, capability: uint32) -> bool;

pub open spec fn QosAtPlatformDefault(s: S, domain_id: uint32, capability: uint32) -> bool;

pub open spec fn QosValue(s: S, domain_id: uint32, capability: uint32) -> uint32;

} // verus!
