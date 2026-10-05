use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type CoreId = u64;

pub enum AffinityState {
    On,
    Off,
    OnPending,
}

pub enum PsciStatusCode {
    Success,
    NotSupported,
    InvalidParameters,
    Denied,
    AlreadyOn,
    OnPending,
    InternalFailure,
    NotPresent,
    Disabled,
    InvalidAddress,
}

pub struct S {
    pub psci_version: f64,
    pub supports_affinity_above_zero: bool,
}

pub spec const ON: Result<AffinityState, PsciStatusCode> = Ok(AffinityState::On);

pub spec const OFF: Result<AffinityState, PsciStatusCode> = Ok(AffinityState::Off);

pub spec const ON_PENDING: Result<AffinityState, PsciStatusCode> = Ok(AffinityState::OnPending);

pub spec const DISABLED: Result<AffinityState, PsciStatusCode> = Err(PsciStatusCode::Disabled);

pub spec const INVALID_PARAMETERS: Result<AffinityState, PsciStatusCode> = Err(PsciStatusCode::InvalidParameters);

pub open spec fn IsAffinityInstancePresent(s: S, lowest_affinity_level: UInt64, target_affinity: UInt64) -> bool;

pub open spec fn IsAffinityInstanceDisabled(s: S, lowest_affinity_level: UInt64, target_affinity: UInt64) -> bool;

pub open spec fn SupportsAffinityLevelAboveZero(s: S) -> bool;

pub open spec fn PsciVersion(s: S) -> f64;

pub open spec fn ResultEqual(a: Result<AffinityState, PsciStatusCode>, b: Result<AffinityState, PsciStatusCode>) -> bool;

pub open spec fn AffinityInstance(s: S, lowest_affinity_level: UInt64, target_affinity: UInt64) -> Set<CoreId>;

pub open spec fn CoreEnabledByCpuOn(s: S, core: CoreId) -> bool;

pub open spec fn IsColdBootPrimaryCore(s: S, core: CoreId) -> bool;

pub open spec fn CoreCalledCpuOff(s: S, core: CoreId) -> bool;

pub open spec fn CpuOffProcessed(s: S, core: CoreId) -> bool;

pub open spec fn CoreState(s: S, core: CoreId) -> Result<AffinityState, PsciStatusCode>;

} // verus!
