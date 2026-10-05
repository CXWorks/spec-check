use vstd::prelude::*;
verus! {

pub type MPIDR = u64;
pub type Address = u64;
pub type RmiStatusCode = u64;
pub type CoreStateCode = u64;

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Result<T, E> {
    pub open spec fn is_Ok(&self) -> bool {
        self is Ok
    }

    pub open spec fn is_Err(&self) -> bool {
        self is Err
    }
}

pub struct Core {
    pub state: CoreStateCode,
    pub power: bool,
    pub caches: int,
}

pub struct S {
    pub cores: Map<MPIDR, Core>,
}

pub const ON: u64 = 1;
pub const ON_PENDING: u64 = 2;
pub const INVALID_PARAMETERS: u64 = 3;
pub const INVALID_ADDRESS: u64 = 4;
pub const ALREADY_ON: u64 = 5;
pub const INTERNAL_FAILURE: u64 = 6;
pub const DENIED: u64 = 7;

pub open spec fn IsValidMpidr(s: S, target_cpu: MPIDR) -> bool;

pub open spec fn ResultEqual(result: Result<(), RmiStatusCode>, code: RmiStatusCode) -> bool;

pub open spec fn IsKnownUnavailableToCaller(s: S, addr: Address) -> bool;

pub open spec fn CoreAt(s: S, target_cpu: MPIDR) -> Core;

pub open spec fn CanPowerUpPhysically(s: S, target_cpu: MPIDR) -> bool;

pub open spec fn IsAvailableForOsUse(s: S, target_cpu: MPIDR) -> bool;

pub open spec fn ContextIdRegister(s: S, core: Core) -> int;

pub open spec fn CachesInvalidatedOnBoot(s: S, target_cpu: MPIDR) -> bool;

pub open spec fn CoherencyManaged(s: S, target_cpu: MPIDR) -> bool;

} // verus!
