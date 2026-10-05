use vstd::prelude::*;
verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;
pub type UInt64 = u64;
pub type Int32 = i32;
pub type FfaFunctionId = u32;
pub type FfaInstance = u8;
pub type CallerId = u16;
pub type Binding = u64;

pub struct S {
    pub dummy: u64,
}

pub spec const FFA_NOTIFICATION_UNBIND2: FfaFunctionId = 0x84000080u32;

pub spec const FFA_SUCCESS: Result<UInt32, Int32> = Ok(0u32);
pub spec const NOT_SUPPORTED: Result<UInt32, Int32> = Err(-1i32);
pub spec const INVALID_PARAMETERS: Result<UInt32, Int32> = Err(-2i32);
pub spec const DENIED: Result<UInt32, Int32> = Err(-6i32);
pub spec const ABORTED: Result<UInt32, Int32> = Err(-8i32);

pub open spec fn IsImplementedAtInstance(s: S, fid: FfaFunctionId, inst: FfaInstance) -> bool;
pub open spec fn CurrentFfaInstance(s: S) -> FfaInstance;
pub open spec fn ResultEqual(r1: Result<UInt32, Int32>, r2: Result<UInt32, Int32>) -> bool;
pub open spec fn IsValidPartitionId(s: S, id: UInt16) -> bool;
pub open spec fn IsValidNotificationBitmap(s: S, bitmap: [UInt64; 6]) -> bool;
pub open spec fn NotificationBitmapBit(s: S, bitmap: [UInt64; 6], i: UInt64) -> UInt64;
pub open spec fn NumSupportedNotifications(s: S) -> UInt64;
pub open spec fn NotificationIsBound(s: S, receiver: UInt16, i: UInt64) -> bool;
pub open spec fn NotificationBoundSender(s: S, receiver: UInt16, i: UInt64) -> UInt16;
pub open spec fn NotificationIsPending(s: S, receiver: UInt16, i: UInt64) -> bool;
pub open spec fn CallerMayInvokeNotificationUnbind2(s: S, caller: CallerId) -> bool;
pub open spec fn CurrentCaller(s: S) -> CallerId;
pub open spec fn PartitionHasAborted(s: S, id: UInt16) -> bool;
pub open spec fn SenderCanSignalNotification(s: S, sender: UInt16, receiver: UInt16, i: UInt64) -> bool;
pub open spec fn NotificationBinding(s: S, receiver: UInt16, i: UInt64) -> Binding;

} // verus!
