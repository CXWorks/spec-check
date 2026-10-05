use vstd::prelude::*;

verus! {

pub type int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: int32 = 0;
pub const NOT_SUPPORTED: int32 = -1;
pub const INVALID_PARAMETERS: int32 = -2;
pub const DENIED: int32 = -3;
pub const NOT_FOUND: int32 = -4;

pub const domain_id: UInt32 = 0;
pub const cpli: UInt32 = 1;
pub const agent: UInt32 = 2;
pub const power_cap: UInt32 = 3;
pub const POWERCAP_CAP_SET: UInt32 = 4;
pub const POWERCAP_CAP_SET_COMPLETE: UInt32 = 5;

pub const flags: [UInt32; 2] = [0, 1];

pub open spec fn ResultEqual(status: int32, code: int32) -> bool;
pub open spec fn IsValidPowercapDomain(s: S, d: UInt32) -> bool;
pub open spec fn IsValidCpli(s: S, d: UInt32, c: UInt32) -> bool;
pub open spec fn IsRequestSupported(s: S, d: UInt32, c: UInt32, f: [UInt32; 2], p: UInt32) -> bool;
pub open spec fn IsSupportedPowerCap(s: S, d: UInt32, c: UInt32, p: UInt32) -> bool;
pub open spec fn IsValidCapSetFlags(f: [UInt32; 2]) -> bool;
pub open spec fn AgentMaySetPowerCap(a: UInt32, d: UInt32) -> bool;
pub open spec fn RequestedPowerCap(a: UInt32, d: UInt32, c: UInt32) -> UInt32;
pub open spec fn CommandQueued(cmd: UInt32, d: UInt32, c: UInt32, p: UInt32) -> bool;
pub open spec fn DelayedResponseSent(msg: UInt32) -> bool;

} // verus!
