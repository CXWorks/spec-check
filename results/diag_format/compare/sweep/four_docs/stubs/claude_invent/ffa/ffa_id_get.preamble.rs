use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt16 = u16;
pub type FfaReturnCode = i32;

pub const FFA_SUCCESS: FfaReturnCode = 0;
pub const NOT_SUPPORTED: FfaReturnCode = -1;

pub struct S {
    pub caller_ffa_id: u16,
    pub ffa_id_get_implemented: bool,
    pub is_non_secure_physical_instance: bool,
}

pub open spec fn FfaIdGetImplemented(s: S) -> bool;

pub open spec fn CallerFfaId(s: S) -> UInt16;

pub open spec fn IsNonSecurePhysicalFfaInstance(s: S) -> bool;

} // verus!
