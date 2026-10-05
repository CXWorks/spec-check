use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

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
}

pub enum RmiStatusCode {
    RmiSuccess,
    RmiErrorInput,
    RmiErrorRealm,
    RmiErrorRec,
    RmiErrorRtt,
    RmiErrorDevice,
    RmiErrorNotSupported,
}

pub struct PowerCapDomainState {
    pub power_cap: UInt32,
}

pub struct S {
    pub dummy: int,
}

pub open spec fn PowerCapDomain(s: S, domain_id: UInt32) -> PowerCapDomainState;

pub open spec fn DomainSupportsCpc(s: S, domain_id: UInt32) -> bool;

} // verus!
