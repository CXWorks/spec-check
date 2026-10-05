use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type UInt64 = u64;

pub type PsciReturnCode = i32;

pub type PsciFunctionId = u32;

pub struct S {
    pub dummy: u64,
}

pub const NOT_SUPPORTED: PsciReturnCode = -1;

pub const INVALID_PARAMETERS: PsciReturnCode = -2;

pub const SYSTEM_RESET2: PsciFunctionId = 0x84000012;

pub const SYSTEM_WARM_RESET: UInt32 = 0;

pub open spec fn IsImplemented(fid: PsciFunctionId) -> bool;

pub open spec fn ResultEqual(a: PsciReturnCode, b: PsciReturnCode) -> bool;

pub open spec fn MainMemoryPreserved() -> bool;

pub open spec fn FellBackToColdReset() -> bool;

pub open spec fn AllMemoryRequestersReset() -> bool;

pub open spec fn AllCpusAndMmusReset() -> bool;

pub open spec fn AllInterruptsDisabled() -> bool;

pub open spec fn SmmuStateEqualsColdResetState() -> bool;

pub open spec fn CookieIgnored(cookie: UInt64) -> bool;

} // verus!
