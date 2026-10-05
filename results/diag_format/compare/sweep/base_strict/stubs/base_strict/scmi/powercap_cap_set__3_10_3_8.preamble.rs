use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const DENIED: Int32 = -3;
pub const NOT_FOUND: Int32 = -4;

pub const POWERCAP_CAP_SET_COMPLETE: UInt32 = 100;

#[allow(non_upper_case_globals)]
pub const domain_id: UInt32 = 1;
#[allow(non_upper_case_globals)]
pub const cpli: UInt32 = 2;
#[allow(non_upper_case_globals)]
pub const power_cap: UInt32 = 3;
#[allow(non_upper_case_globals)]
pub const flags: UInt32 = 4;
#[allow(non_upper_case_globals)]
pub const caller: UInt32 = 5;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn Bits(x: UInt32, hi: UInt32, lo: UInt32) -> UInt32;

pub open spec fn IsValidPowercapDomain(s: S, domain: UInt32) -> bool;

pub open spec fn IsValidCpli(s: S, domain: UInt32, cpli_: UInt32) -> bool;

pub open spec fn IsPowercapCapSetSupported(s: S, domain: UInt32, cpli_: UInt32) -> bool;

pub open spec fn IsSupportedPowerCap(s: S, domain: UInt32, cpli_: UInt32, cap: UInt32) -> bool;

pub open spec fn IsValidPowercapCapSetFlags(s: S, f: UInt32) -> bool;

pub open spec fn AgentMaySetPowerCap(s: S, agent: UInt32, domain: UInt32) -> bool;

pub open spec fn AgentRequestedPowerCap(s: S, agent: UInt32, domain: UInt32, cpli_: UInt32) -> UInt32;

pub open spec fn AgentPowerCapEnabled(s: S, agent: UInt32, domain: UInt32, cpli_: UInt32) -> bool;

pub open spec fn PowerCapSettingCompleted(s: S, domain: UInt32, cpli_: UInt32, cap: UInt32) -> bool;

pub open spec fn PowerCapSetRequestQueued(s: S, domain: UInt32, cpli_: UInt32, cap: UInt32) -> bool;

pub open spec fn DelayedResponsePending(msg: UInt32, domain: UInt32) -> bool;

} // verus!
