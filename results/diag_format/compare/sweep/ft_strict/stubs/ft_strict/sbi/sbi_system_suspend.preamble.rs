use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type Address = u64;
pub type SbiErrorCode = i64;
pub type HartIndex = u64;
pub type HartState = u32;
pub type PrivilegeMode = u32;

pub struct S {
    pub dummy: int,
}

pub struct Sstatus {
    pub SIE: u64,
    pub SPIE: u64,
    pub SPP: u64,
}

pub spec const SBI_SUCCESS: SbiErrorCode = 0;
pub spec const SBI_ERR_FAILED: SbiErrorCode = (-1) as i64;

pub spec const STARTED: HartState = 0;
pub spec const STOPPED: HartState = 1;
pub spec const SUSPENDED: HartState = 2;

pub spec const USER: PrivilegeMode = 0;
pub spec const SUPERVISOR: PrivilegeMode = 1;
pub spec const MACHINE: PrivilegeMode = 3;

pub open spec fn CallingHart(s: S) -> HartIndex;
pub open spec fn HartResumesFromState(s: S, hart: HartIndex, state: HartState) -> bool;
pub open spec fn HartPrivilegeMode(s: S, hart: HartIndex) -> PrivilegeMode;
pub open spec fn HartResumesAtAddress(s: S, hart: HartIndex, addr: Address) -> bool;
pub open spec fn satp(s: S) -> u64;
pub open spec fn sstatus(s: S) -> Sstatus;
pub open spec fn a0(s: S) -> UInt64;
pub open spec fn a1(s: S) -> UInt64;
pub open spec fn HartId(s: S, hart: HartIndex) -> UInt64;
pub open spec fn OtherRegistersUndefined(s: S, hart: HartIndex) -> bool;

} // verus!
