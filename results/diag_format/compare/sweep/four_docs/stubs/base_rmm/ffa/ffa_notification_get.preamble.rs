use vstd::prelude::*;
verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;
pub type UInt64 = u64;
pub type FuncId = u32;
pub type Instance = u8;
pub type NotificationClass = u8;

pub struct S {
    pub sp_bitmap_lo_field: UInt64,
    pub sp_bitmap_hi_field: UInt32,
    pub vm_bitmap_lo_field: UInt64,
    pub vm_bitmap_hi_field: UInt32,
    pub spm_bitmap_field: UInt64,
    pub hyp_bitmap_field: UInt64,
}

pub const FFA_SUCCESS: UInt32 = 0;
pub const NOT_SUPPORTED: UInt32 = 1;
pub const DENIED: UInt32 = 2;
pub const INVALID_PARAMETERS: UInt32 = 3;

pub const FFA_NOTIFICATION_GET: FuncId = 0x84000082;

pub const NS_PHYSICAL: Instance = 0;
pub const NS_VIRTUAL: Instance = 1;
pub const SECURE_PHYSICAL: Instance = 2;
pub const SECURE_VIRTUAL: Instance = 3;

pub const SP: NotificationClass = 0;
pub const VM: NotificationClass = 1;
pub const SPM_FRAMEWORK: NotificationClass = 2;
pub const HYP_FRAMEWORK: NotificationClass = 3;

pub open spec fn IsImplementedAtInstance(func: FuncId, inst: Instance) -> bool;
pub open spec fn CurrentInstance() -> Instance;
pub open spec fn ResultEqual(result: UInt32, code: UInt32) -> bool;
pub open spec fn CallerMayInvoke(func: FuncId) -> bool;
pub open spec fn IsRecognizedPartitionId(id: UInt16) -> bool;
pub open spec fn sp_bitmap_hi(s: S) -> UInt32;
pub open spec fn sp_bitmap_lo(s: S) -> UInt64;
pub open spec fn vm_bitmap_hi(s: S) -> UInt32;
pub open spec fn vm_bitmap_lo(s: S) -> UInt64;
pub open spec fn spm_bitmap(s: S) -> UInt64;
pub open spec fn hyp_bitmap(s: S) -> UInt64;
pub open spec fn PendingNotifications(receiver_id: UInt16, receiver_vcpu_id: UInt16, class: NotificationClass) -> UInt64;

} // verus!
