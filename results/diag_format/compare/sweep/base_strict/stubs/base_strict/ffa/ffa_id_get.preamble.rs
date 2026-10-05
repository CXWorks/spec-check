use vstd::prelude::*;
verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;
pub type Int32 = i32;

pub enum FfaInstance {
    NonSecurePhysical,
    NonSecureVirtual,
    SecurePhysical,
    SecureVirtual,
}

pub struct S {
    pub caller_id: UInt16,
    pub instance: FfaInstance,
}

pub const FFA_SUCCESS: UInt32 = 0x84000061u32;
pub const FFA_ID_GET: UInt32 = 0x84000069u32;
pub const NOT_SUPPORTED: Int32 = -1i32;

pub open spec fn IsFunctionImplementedAtInstance(func_id: UInt32, instance: FfaInstance) -> bool;
pub open spec fn CurrentFfaInstance() -> FfaInstance;
pub open spec fn ResultEqual<T>(a: T, b: T) -> bool;
pub open spec fn CallerFfaId() -> UInt16;
pub open spec fn IsNonSecurePhysicalFfaInstance(instance: FfaInstance) -> bool;

} // verus!
