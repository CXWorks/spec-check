use vstd::prelude::*;

verus! {

pub type PsciReturnCode = i64;
pub type CoreStateValue = i64;
pub type PowerState = u64;
pub type CacheState = u64;

pub struct S {
    pub dummy: u64,
}

pub spec const INVALID_PARAMETERS: i64 = (-2int) as i64;
pub spec const DENIED: i64 = (-3int) as i64;
pub spec const ALREADY_ON: i64 = (-4int) as i64;
pub spec const ON_PENDING: i64 = (-5int) as i64;
pub spec const INTERNAL_FAILURE: i64 = (-6int) as i64;
pub spec const INVALID_ADDRESS: i64 = (-9int) as i64;
pub spec const ON: i64 = 0i64;

#[allow(non_upper_case_globals)]
pub spec const target_cpu: u64 = 1;
#[allow(non_upper_case_globals)]
pub spec const entry_point_address: u64 = 2;
#[allow(non_upper_case_globals)]
pub spec const context_id: u64 = 3;

pub uninterp spec fn IsValidMpidr(s: S, cpu: u64) -> bool;
pub uninterp spec fn IsKnownUnavailableToCaller(s: S, addr: u64) -> bool;
pub uninterp spec fn CoreState(s: S, cpu: u64) -> CoreStateValue;
pub uninterp spec fn CanBePhysicallyPoweredUp(s: S, cpu: u64) -> bool;
pub uninterp spec fn IsAvailableForOsUsage(s: S, cpu: u64) -> bool;
pub uninterp spec fn ResultEqual(result: PsciReturnCode, code: PsciReturnCode) -> bool;
pub uninterp spec fn CoreRestartsAtEntryPoint(s: S, cpu: u64, addr: u64) -> bool;
pub uninterp spec fn ContextIdPresentedAtReturnExceptionLevel(s: S, cpu: u64, ctx: u64) -> bool;
pub uninterp spec fn CachesInvalidatedOnBoot(s: S, cpu: u64) -> bool;
pub uninterp spec fn HardwareInvalidatesCachesOnBoot(s: S, cpu: u64) -> bool;
pub uninterp spec fn CoherencyManaged(s: S, cpu: u64) -> bool;
pub uninterp spec fn CorePowerState(s: S, cpu: u64) -> PowerState;
pub uninterp spec fn CoreCaches(s: S, cpu: u64) -> CacheState;

} // verus!
