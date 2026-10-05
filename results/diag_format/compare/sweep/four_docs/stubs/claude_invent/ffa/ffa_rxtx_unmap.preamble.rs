use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type FfaErrorCode = i32;

pub const NOT_SUPPORTED: FfaErrorCode = -1;
pub const INVALID_PARAMETERS: FfaErrorCode = -2;

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

    pub open spec fn get_Err_0(self) -> E {
        match self {
            Result::Err(e) => e,
            Result::Ok(_) => arbitrary(),
        }
    }
}

pub struct S {
    pub dummy: int,
}

pub open spec fn FfaRxtxUnmapImplemented(s: S) -> bool;

pub open spec fn FfaRxtxUnmapCallerId(s: S, id: UInt32) -> UInt32;

pub open spec fn RxTxBufferPairRegistered(s: S, caller: UInt32) -> bool;

pub open spec fn RxTxBuffersUnchangedExcept(old_s: S, new_s: S, caller: UInt32) -> bool;

} // verus!
