use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type PsciReturnCode = i32;

pub const NOT_SUPPORTED: PsciReturnCode = -1;
pub const INVALID_PARAMETERS: PsciReturnCode = -2;

pub struct S {
    pub dummy: int,
}

pub open spec fn IsSystemOff2Implemented(s: S) -> bool;

pub open spec fn SystemOff2ParamsValid(s: S, hibernate_type: UInt32, cookie: UInt64) -> bool;

pub open spec fn SystemHibernateStateSaved(old_s: S, new_s: S) -> bool;

pub open spec fn SystemPoweredOff(s: S) -> bool;

} // verus!
