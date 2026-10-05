use vstd::prelude::*;

verus! {

pub type FuncId = u32;
pub type InstanceId = u32;
pub type ResultCode = u32;
pub type PartitionId = u64;

pub struct S {
    pub dummy: u64,
}

pub const FFA_NOTIFICATION_BIND2: FuncId = 0x8400007F;

pub const FFA_SUCCESS: ResultCode = 0;
pub const NOT_SUPPORTED: ResultCode = 1;
pub const INVALID_PARAMETERS: ResultCode = 2;
pub const DENIED: ResultCode = 3;
pub const ABORTED: ResultCode = 4;

pub open spec fn IsImplementedAtInstance(func: FuncId, inst: InstanceId) -> bool;

pub open spec fn CurrentInstance() -> InstanceId;

pub open spec fn ResultEqual(result: u32, code: ResultCode) -> bool;

pub open spec fn Bits<T>(x: T, hi: u64, lo: u64) -> u64;

pub open spec fn IsValidSenderId(id: PartitionId) -> bool;

pub open spec fn IsValidReceiverId(id: PartitionId) -> bool;

pub open spec fn PerVcpuNotificationsSupported() -> bool;

pub open spec fn NumSupportedNotifications() -> u64;

pub open spec fn BitmapBit<R>(bitmap: u64, i: u64) -> R;

pub open spec fn IsBoundToOtherSender(receiver: PartitionId, i: u64, sender: PartitionId) -> bool;

pub open spec fn IsNotificationPending(receiver: PartitionId, i: u64) -> bool;

pub open spec fn CallerAllowedToInvoke(func: FuncId) -> bool;

pub open spec fn PartitionHasAborted(id: PartitionId) -> bool;

pub open spec fn NotificationBoundSender(receiver: PartitionId, i: u64) -> PartitionId;

pub open spec fn NotificationIsPerVcpu(receiver: PartitionId, i: u64) -> bool;

} // verus!
