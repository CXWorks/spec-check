use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type Int32 = i32;

pub type UInt64 = u64;

pub type CallerId = u64;

pub enum RsiCommandReturnCode {
    Success,
    NotFound,
    InvalidParameters,
    Denied,
}

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: RsiCommandReturnCode = RsiCommandReturnCode::Success;

pub const NOT_FOUND: RsiCommandReturnCode = RsiCommandReturnCode::NotFound;

pub const INVALID_PARAMETERS: RsiCommandReturnCode = RsiCommandReturnCode::InvalidParameters;

pub spec const RSI_SUCCESS: Result<(), RsiCommandReturnCode> = Ok(());

#[allow(non_upper_case_globals)]
pub spec const caller: CallerId = 0;

pub open spec fn IsValidPowercapDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn IsValidNotifyEnable(s: S, notify_enable: UInt32) -> bool;

pub open spec fn ResultEqual(result: Result<(), RsiCommandReturnCode>, code: RsiCommandReturnCode) -> bool;

pub open spec fn Bits(value: UInt32, hi: int, lo: int) -> int;

pub open spec fn CapChangeNotifyEnabled(s: S, agent: CallerId, domain_id: UInt32) -> bool;

} // verus!
