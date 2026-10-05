use vstd::prelude::*;

verus! {

pub type UInt64 = u64;
pub type UInt32 = u32;
pub type Int32 = i32;
pub type UInt16 = u16;

pub type FuncId = u32;
pub type InstanceId = u32;
pub type ResultCode = u32;
pub type PartitionId = u16;

pub struct S {
    pub cmd_input_1: UInt64,
    pub cmd_input_2: UInt64,
    pub cmd_input_3: UInt64,
    pub cmd_input_9: UInt64,
    pub cmd_input_15: UInt64,
    pub cmd_input_16: UInt64,
}

pub const FFA_NOTIFICATION_GET2: FuncId = 0x8400_0090u32;

pub const NS_PHYSICAL: InstanceId = 1u32;

pub const NOT_SUPPORTED: ResultCode = 0xFFFF_FFFFu32;
pub const INVALID_PARAMETERS: ResultCode = 0xFFFF_FFFEu32;
pub const DENIED: ResultCode = 0xFFFF_FFFDu32;
pub const FFA_SUCCESS64: ResultCode = 0xC400_0061u32;

pub open spec fn IsImplementedAtInstance(func: FuncId, inst: InstanceId) -> bool;

pub open spec fn CurrentInstance() -> InstanceId;

pub open spec fn ResultEqual(result: UInt32, code: ResultCode) -> bool;

pub open spec fn CallerAllowedToInvoke(caller: PartitionId, func: FuncId) -> bool;

pub open spec fn Caller() -> PartitionId;

pub open spec fn IsRecognizedPartitionId(id: UInt64) -> bool;

pub open spec fn Bits64(value: UInt64, hi: int, lo: int) -> UInt64;

pub open spec fn ExceedsSupportedNotifications(bitmap: UInt64) -> bool;

pub open spec fn EmptyNotificationBitmapSpecified(flags: UInt64, receiver: UInt64) -> bool;

pub open spec fn RetrievedNotifications<T>(pending: T, mask: UInt64) -> T;

pub open spec fn PendingSpNotifications(receiver: UInt64) -> [UInt64; 6];

pub open spec fn PendingVmNotifications(receiver: UInt64) -> [UInt64; 6];

pub open spec fn PendingSpmFrameworkNotifications(receiver: UInt64) -> UInt64;

pub open spec fn PendingHypFrameworkNotifications(receiver: UInt64) -> UInt64;

pub open spec fn MaskedNotificationsRemainInCurrentState(receiver: UInt64, sp_mask: UInt64, vm_mask: UInt64, spmc_mask: UInt64, hyp_mask: UInt64) -> bool;

} // verus!
