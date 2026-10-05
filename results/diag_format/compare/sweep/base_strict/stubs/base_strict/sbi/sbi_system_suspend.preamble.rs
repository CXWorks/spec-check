use vstd::prelude::*;

verus! {

pub type SbiErrorCode = i64;

pub type Hart = u64;

pub type HartState = u64;

pub type PrivilegeMode = u8;

pub struct S {
    pub dummy: u64,
}

pub struct SstatusReg {
    pub SIE: u64,
}

pub spec const STOPPED: HartState = 1;

pub spec const STARTED: HartState = 2;

pub spec const SUPERVISOR: PrivilegeMode = 1;

pub spec const USER: PrivilegeMode = 0;

pub spec const MACHINE: PrivilegeMode = 3;

pub spec const satp: u64 = 0;

pub spec const resume_addr: u64 = 11;

pub spec const a0: u64 = 12;

pub spec const a1: u64 = 13;

pub spec const opaque: u64 = 14;

pub spec const sstatus: SstatusReg = SstatusReg { SIE: 0 };

pub open spec fn CallingHart() -> Hart;

pub open spec fn HartResumesFromState(s: S, hart: Hart, state: HartState) -> bool;

pub open spec fn HartPrivilegeMode(s: S, hart: Hart) -> PrivilegeMode;

pub open spec fn HartResumesAtAddress(s: S, hart: Hart, addr: u64) -> bool;

pub open spec fn HartId(hart: Hart) -> u64;

pub open spec fn OtherRegistersUndefined(hart: Hart) -> bool;

} // verus!
