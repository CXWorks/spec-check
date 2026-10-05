use vstd::prelude::*;
verus! {

pub type FfaStatusCode = i32;

pub const FFA_SUCCESS: FfaStatusCode = 0;
pub const FFA_NOT_SUPPORTED: FfaStatusCode = -1;

pub const FFA_ID_GET: u32 = 0x84000069;

pub struct S {
    pub regs: Seq<u64>,
}

pub open spec fn IsImplementedAtInstance(func_id: u32) -> bool;

pub open spec fn ResultEqual(result: Result<(), FfaStatusCode>, code: FfaStatusCode) -> bool;

pub open spec fn CallerId() -> u16;

pub open spec fn IsNonSecurePhysicalInstance() -> bool;

pub open spec fn w2(s: S, hi: int, lo: int) -> int;

} // verus!
