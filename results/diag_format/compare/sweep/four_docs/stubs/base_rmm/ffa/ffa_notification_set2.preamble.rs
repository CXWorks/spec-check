use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt16 = u16;
pub type FfaReturnCode = i32;
pub type FfaFunctionId = u32;

pub struct FfaState {
    pub dummy: int,
}

pub const FFA_SUCCESS: FfaReturnCode = 0;
pub const NOT_SUPPORTED: FfaReturnCode = -1;
pub const INVALID_PARAMETERS: FfaReturnCode = -2;
pub const DENIED: FfaReturnCode = -3;
pub const ABORTED: FfaReturnCode = -8;

pub const FFA_NOTIFICATION_SET2: FfaFunctionId = 0x8400_0081;

pub open spec fn IsImplementedAtInstance(s: FfaState, fid: FfaFunctionId) -> bool;

pub open spec fn ResultEqual(a: FfaReturnCode, b: FfaReturnCode) -> bool;

pub open spec fn IsRecognizedPartitionId(s: FfaState, id: UInt16) -> bool;

pub open spec fn sender_id(s: FfaState) -> UInt16;

pub open spec fn receiver_id(s: FfaState) -> UInt16;

pub open spec fn IsValidFlags(s: FfaState) -> bool;

pub open spec fn per_vcpu(s: FfaState) -> char;

pub open spec fn receiver_vcpu_id(s: FfaState) -> UInt32;

pub open spec fn bitmap(s: FfaState) -> Seq<char>;

pub open spec fn BitmapContainsPerVcpuNotification(s: FfaState, receiver: UInt16, bm: Seq<char>) -> bool;

pub open spec fn BitmapContainsGlobalNotification(s: FfaState, receiver: UInt16, bm: Seq<char>) -> bool;

pub open spec fn PerVcpuNotificationsSupported(s: FfaState) -> bool;

pub open spec fn BitmapExceedsSupportedNotifications(s: FfaState, bm: Seq<char>) -> bool;

pub open spec fn IsEmptyBitmap(s: FfaState, bm: Seq<char>) -> bool;

pub open spec fn SenderPermittedToSignalAll(s: FfaState, sender: UInt16, receiver: UInt16, bm: Seq<char>) -> bool;

pub open spec fn SupportsNotificationReceipt(s: FfaState, receiver: UInt16) -> bool;

pub open spec fn HasAborted(s: FfaState, receiver: UInt16) -> bool;

pub open spec fn NotificationSignaled(s: FfaState, receiver: UInt16, vcpu: UInt32, i: int) -> bool;

pub open spec fn NotificationSignalState(s: FfaState, receiver: UInt16, bm: Seq<char>) -> bool;

} // verus!
