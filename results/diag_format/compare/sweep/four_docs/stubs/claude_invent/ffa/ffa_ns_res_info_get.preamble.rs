use vstd::prelude::*;
verus! {

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

pub use crate::Result::{Ok, Err};

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

pub type FfaErrorCode = i32;

pub const NOT_SUPPORTED: FfaErrorCode = -1;
pub const INVALID_PARAMETERS: FfaErrorCode = -2;
pub const RETRY: FfaErrorCode = -7;

pub struct S {
    pub state_id: nat,
}

pub uninterp spec fn IsValidSEndpointId(s: S, id: u16) -> bool;
pub uninterp spec fn FfaNsResInfoGetImplemented(s: S) -> bool;
pub uninterp spec fn CallerRxBufferMappedInCallee(s: S) -> bool;
pub uninterp spec fn CallerRxBufferOwnedByCallee(s: S) -> bool;
pub uninterp spec fn CalleeIsBusy(s: S) -> bool;
pub uninterp spec fn RetrievalAborted(s: S) -> bool;
pub uninterp spec fn ResourceInfoDescriptorReturnedInRx(old_s: S, new_s: S, target_valid: bool, sep_id: u16, request_continue: bool, written_size: u64, remaining_size: u64) -> bool;
pub uninterp spec fn NsPasEntirelyInaccessible(s: S, target_valid: bool, sep_id: u16) -> bool;

} // verus!
