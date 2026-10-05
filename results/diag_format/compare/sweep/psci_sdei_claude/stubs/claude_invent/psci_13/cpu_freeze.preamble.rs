use vstd::prelude::*;

verus! {

pub type PsciReturnCode = i32;

pub const NOT_SUPPORTED: PsciReturnCode = -1;
pub const DENIED: PsciReturnCode = -3;

pub struct S {
    pub cpu_freeze_implemented: bool,
    pub cpu_off_denied: bool,
}

pub open spec fn CpuFreezeImplemented(s: S) -> bool;

pub open spec fn CpuOffDenied(s: S) -> bool;

} // verus!
