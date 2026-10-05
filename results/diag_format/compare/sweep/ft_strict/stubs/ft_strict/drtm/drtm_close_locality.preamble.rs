use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub enum NotSupported {
    NotSupported,
    InvalidParameters,
    AlreadyClosed,
    Denied,
}

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Result<T, E> {
    pub open spec fn is_Ok(self) -> bool {
        match self {
            Result::Ok(_) => true,
            Result::Err(_) => false,
        }
    }

    pub open spec fn is_Err(self) -> bool {
        match self {
            Result::Ok(_) => false,
            Result::Err(_) => true,
        }
    }
}

pub spec const SUCCESS: Result<(), NotSupported> = Result::Ok(());
pub spec const NOT_SUPPORTED: Result<(), NotSupported> = Result::Err(NotSupported::NotSupported);
pub spec const INVALID_PARAMETERS: Result<(), NotSupported> = Result::Err(NotSupported::InvalidParameters);
pub spec const ALREADY_CLOSED: Result<(), NotSupported> = Result::Err(NotSupported::AlreadyClosed);
pub spec const DENIED: Result<(), NotSupported> = Result::Err(NotSupported::Denied);

pub open spec fn DrtmIsSupported(s: S) -> bool;

pub open spec fn ResultEqual(a: Result<(), NotSupported>, b: Result<(), NotSupported>) -> bool;

pub open spec fn LocalityIsClosed(s: S, locality: UInt32) -> bool;

pub open spec fn LocalityIsRelinquished(s: S, locality: UInt32) -> bool;

} // verus!
