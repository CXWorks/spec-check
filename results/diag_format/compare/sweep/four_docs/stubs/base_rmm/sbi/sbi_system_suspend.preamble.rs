use vstd::prelude::*;
verus! {

pub type SbiErrorCode = i64;
pub type HartState = u64;
pub type PrivilegeMode = u64;

pub const SBI_SUCCESS: SbiErrorCode = 0;
pub const SBI_ERR_FAILED: SbiErrorCode = -1;
pub const SBI_ERR_NOT_SUPPORTED: SbiErrorCode = -2;

pub const STARTED: HartState = 0;
pub const STOPPED: HartState = 1;
pub const SUSPENDED: HartState = 2;

pub const USER: PrivilegeMode = 0;
pub const SUPERVISOR: PrivilegeMode = 1;
pub const MACHINE: PrivilegeMode = 3;

pub struct Sstatus {
    pub SIE: u64,
}

pub struct S {
    pub resume_addr: u64,
    pub hartid: u64,
    pub opaque: u64,
}

pub open spec fn HartResumedFrom(s: S, st: HartState) -> bool;
pub open spec fn CurrentPrivilegeMode(s: S) -> PrivilegeMode;
pub open spec fn pc(s: S) -> u64;
pub open spec fn satp(s: S) -> u64;
pub open spec fn sstatus(s: S) -> Sstatus;
pub open spec fn a0(s: S) -> u64;
pub open spec fn a1(s: S) -> u64;

} // verus!
