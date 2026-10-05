use vstd::prelude::*;
verus! {

pub type UInt64 = u64;
pub type MPIDR = u64;
pub type Address = u64;
pub type RmiStatusCode = u64;
pub type CoreStateCode = u64;

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Result<T, E> {
    pub open spec fn is_Ok(&self) -> bool;
}

pub struct Caches {
    pub enabled: bool,
}

pub struct Core {
    pub state: CoreStateCode,
    pub power: bool,
    pub caches: Caches,
}

pub struct S {
    pub core_caches: Caches,
}

pub const SUCCESS: u64 = 0;
pub const INVALID_PARAMETERS: u64 = 1;
pub const INVALID_ADDRESS: u64 = 2;
pub const ALREADY_ON: u64 = 3;
pub const ON_PENDING: u64 = 4;
pub const INTERNAL_FAILURE: u64 = 5;
pub const DENIED: u64 = 6;
pub const ON: u64 = 7;
pub const OFF: u64 = 8;

pub open spec fn IsValidMpidr(target_cpu: MPIDR) -> bool;
pub open spec fn ResultEqual(result: Result<(), RmiStatusCode>, code: u64) -> bool;
pub open spec fn IsKnownUnavailableToCaller(entry_point_address: Address) -> bool;
pub open spec fn CoreAt(s: S, target_cpu: MPIDR) -> Core;
pub open spec fn CanPowerUpPhysically(target_cpu: MPIDR) -> bool;
pub open spec fn IsAvailableForOsUse(target_cpu: MPIDR) -> bool;

} // verus!
