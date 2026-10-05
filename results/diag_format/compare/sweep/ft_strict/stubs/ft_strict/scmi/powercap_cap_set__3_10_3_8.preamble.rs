use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = (-1int) as i32;
pub spec const INVALID_PARAMETERS: Int32 = (-2int) as i32;
pub spec const DENIED: Int32 = (-3int) as i32;
pub spec const NOT_FOUND: Int32 = (-4int) as i32;
pub spec const result: Int32 = (-100int) as i32;

pub spec const POWERCAP_CAP_SET_COMPLETE: UInt32 = 6;
pub spec const caller: UInt32 = 1000;

pub uninterp spec fn IsValidPowercapDomain(s: S, domain_id: UInt32) -> bool;
pub uninterp spec fn IsValidCpli(s: S, domain_id: UInt32, cpli: UInt32) -> bool;
pub uninterp spec fn IsPowercapCapSetSupported(s: S, domain_id: UInt32, cpli: UInt32) -> bool;
pub uninterp spec fn IsSupportedPowerCap(s: S, domain_id: UInt32, cpli: UInt32, power_cap: UInt32) -> bool;
pub uninterp spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt32;
pub uninterp spec fn IsValidPowercapCapSetFlags(s: S, flags: UInt32) -> bool;
pub uninterp spec fn AgentMaySetPowerCap(s: S, agent: UInt32, domain_id: UInt32) -> bool;
pub uninterp spec fn ResultEqual(status: Int32, code: Int32) -> bool;
pub uninterp spec fn AgentRequestedPowerCap(s: S, agent: UInt32, domain_id: UInt32, cpli: UInt32) -> UInt32;
pub uninterp spec fn AgentPowerCapEnabled(s: S, agent: UInt32, domain_id: UInt32, cpli: UInt32) -> bool;
pub uninterp spec fn PowerCapSettingCompleted(s: S, domain_id: UInt32, cpli: UInt32, power_cap: UInt32) -> bool;
pub uninterp spec fn PowerCapSetRequestQueued(s: S, domain_id: UInt32, cpli: UInt32, power_cap: UInt32) -> bool;
pub uninterp spec fn DelayedResponsePending(s: S, msg_id: UInt32, domain_id: UInt32) -> bool;

} // verus!
