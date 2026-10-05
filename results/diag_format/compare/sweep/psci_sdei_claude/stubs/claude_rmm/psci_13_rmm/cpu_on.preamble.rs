use vstd::prelude::*;

verus! {

pub type UInt64 = u64;
pub type MPIDR = u64;
pub type Address = u64;
pub type PsciReturnCode = i32;
pub type CoreState = i32;

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Result<T, E> {
    pub open spec fn is_Ok(self) -> bool {
        match self {
            Result::Ok(_) => true,
            Result::Err(_) => false,
        }
    }

    pub open spec fn is_Err(self) -> bool {
        match self {
            Result::Ok(_) => false,
            Result::Err(_) => true,
        }
    }
}

pub const INVALID_PARAMETERS: i32 = -2;
pub const INVALID_ADDRESS: i32 = -9;
pub const ALREADY_ON: i32 = -4;
pub const ON_PENDING: i32 = -5;
pub const INTERNAL_FAILURE: i32 = -6;
pub const DENIED: i32 = -3;
pub const ON: i32 = 100;
pub const OFF: i32 = 101;

pub struct Core {
    pub state: CoreState,
    pub context_id: UInt64,
}

pub struct S {
    pub cores: Map<MPIDR, Core>,
}

pub open spec fn IsValidMpidr(cpu: MPIDR) -> bool;

pub open spec fn ResultEqual(result: Result<(), PsciReturnCode>, code: i32) -> bool;

pub open spec fn IsKnownUnavailableToCaller(addr: Address) -> bool;

pub open spec fn CoreAt(s: S, cpu: MPIDR) -> Core;

pub open spec fn CanPowerUpPhysically(cpu: MPIDR) -> bool;

pub open spec fn IsAvailableForOsUse(cpu: MPIDR) -> bool;

pub open spec fn ContextIdRegister(core: Core) -> UInt64;

pub open spec fn CachesInvalidatedOnBoot(cpu: MPIDR) -> bool;

pub open spec fn CoherencyManaged(cpu: MPIDR) -> bool;

} // verus!
