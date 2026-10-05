use vstd::prelude::*;
verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;
pub type Int32 = i32;

pub struct FfaInstance {
    pub id: nat,
}

pub struct S {
    pub current_instance: FfaInstance,
}

pub const FFA_SUCCESS: UInt32 = 0x84000061u32;
pub const FFA_ID_GET: UInt32 = 0x84000069u32;

pub const NOT_SUPPORTED: Int32 = -1i32;

pub open spec fn IsFunctionImplementedAtInstance(s: S, func_id: UInt32, inst: FfaInstance) -> bool;

pub open spec fn CurrentFfaInstance(s: S) -> FfaInstance;

pub open spec fn ResultEqual<A, B>(a: A, b: B) -> bool;

pub open spec fn CallerFfaId() -> UInt16;

pub open spec fn IsNonSecurePhysicalFfaInstance(s: S, inst: FfaInstance) -> bool;

} // verus!
