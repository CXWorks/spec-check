use vstd::prelude::*;

verus! {

pub type PsciReturnCode = i32;

pub struct S {
    pub dummy: u64,
}

pub const NOT_SUPPORTED: PsciReturnCode = -1;
pub const INVALID_PARAMETERS: PsciReturnCode = -2;

pub open spec fn IsImplemented(fid: u32) -> bool;

pub open spec fn ResultEqual(result: PsciReturnCode, code: PsciReturnCode) -> bool;

pub open spec fn MainMemoryPreserved() -> bool;

pub open spec fn FellBackToColdReset() -> bool;

pub open spec fn AllMemoryRequestersReset() -> bool;

pub open spec fn AllCpusAndMmusReset() -> bool;

pub open spec fn AllInterruptsDisabled() -> bool;

pub open spec fn SmmuStateEqualsColdResetState() -> bool;

pub open spec fn CookieIgnored(cookie: u64) -> bool;

} // verus!
