use vstd::prelude::*;
verus! {

pub struct S {
    pub regs: Seq<u64>,
}

pub open spec fn Bits(s: S, hi: int, lo: int) -> u64;

pub open spec fn IsRecognizedPartitionId(id: u64) -> bool;

pub open spec fn ResultEqual(result: u32, code: u32) -> bool;

pub const INVALID_PARAMETERS: u32 = 1;
pub const NOT_SUPPORTED: u32 = 2;
pub const DENIED: u32 = 3;
pub const FFA_SUCCESS: u32 = 4;

pub type FfaFunctionId = u32;
pub type FfaInstance = u32;

pub const FFA_NOTIFICATION_BITMAP_DESTROY: FfaFunctionId = 0x8400007E;

pub open spec fn IsImplementedAtInstance(func: FfaFunctionId, inst: FfaInstance) -> bool;

pub open spec fn CurrentFfaInstance() -> FfaInstance;

pub open spec fn IsNotificationBitmapRegistered(id: u64) -> bool;

pub open spec fn IsNotificationBitmapMasked(id: u64) -> bool;

pub open spec fn IsNotificationBitmapPending(id: u64) -> bool;

} // verus!
