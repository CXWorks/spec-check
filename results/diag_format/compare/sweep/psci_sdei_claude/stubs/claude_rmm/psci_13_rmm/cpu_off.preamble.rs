use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub type PsciReturnCode = i64;

pub type CoreId = u64;

pub type PowerStateValue = u64;

pub spec const SUCCESS: PsciReturnCode = 0;
pub spec const NOT_SUPPORTED: PsciReturnCode = (-1int) as i64;
pub spec const INVALID_PARAMETERS: PsciReturnCode = (-2int) as i64;
pub spec const DENIED: PsciReturnCode = (-3int) as i64;

pub spec const POWERED_ON: PowerStateValue = 0;
pub spec const POWERED_DOWN: PowerStateValue = 1;
pub spec const RETENTION: PowerStateValue = 2;

pub struct S {
    pub dummy: u64,
}

pub open spec fn CallingCore(s: S) -> CoreId;

pub open spec fn IsTrustedOsResident(s: S, core: CoreId) -> bool;

pub open spec fn ReturnsToCaller(s: S) -> bool;

pub open spec fn PowerState(s: S, core: CoreId) -> PowerStateValue;

pub open spec fn CachesClean(s: S, core: CoreId) -> bool;

pub open spec fn IsCoherent(s: S, core: CoreId) -> bool;

} // verus!
