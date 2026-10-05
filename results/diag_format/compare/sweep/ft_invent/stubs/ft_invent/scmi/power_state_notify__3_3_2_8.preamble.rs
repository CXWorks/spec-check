use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;

pub enum RsiCommandReturnCode {
    RsiError,
    RsiNotFound,
    RsiInvalidParameters,
}

pub spec const RSI_SUCCESS: Result<(), RsiCommandReturnCode> = Ok(());
pub spec const RSI_NOT_FOUND: Result<(), RsiCommandReturnCode> = Err(RsiCommandReturnCode::RsiNotFound);
pub spec const RSI_INVALID_PARAMETERS: Result<(), RsiCommandReturnCode> = Err(RsiCommandReturnCode::RsiInvalidParameters);

pub struct Realm {
    pub power_state_notify: UInt64,
}

pub struct S {
    pub realms: Map<int, Realm>,
}

pub open spec fn RealmAt(s: S, id: int) -> Realm;

} // verus!
