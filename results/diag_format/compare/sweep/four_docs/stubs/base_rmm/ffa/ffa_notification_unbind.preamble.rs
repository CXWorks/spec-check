use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt16 = u16;
pub type PartitionId = u16;
pub type FunctionId = u32;
pub type NotificationId = nat;

pub struct S {
    pub w1: u32,
    pub w2: u32,
    pub w3: u32,
    pub w4: u32,
}

pub const FFA_NOTIFICATION_UNBIND: FunctionId = 0x84000080u32;

pub const FFA_SUCCESS: Int32 = 0i32;
pub const NOT_SUPPORTED: Int32 = -1i32;
pub const INVALID_PARAMETERS: Int32 = -2i32;
pub const DENIED: Int32 = -6i32;
pub const ABORTED: Int32 = -8i32;

pub open spec fn IsImplementedAtThisInstance(s: S, fid: FunctionId) -> bool;

pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;

pub open spec fn IsRecognizedPartitionId(s: S, id: PartitionId) -> bool;

pub open spec fn sender_id(s: S) -> PartitionId;

pub open spec fn receiver_id(s: S) -> PartitionId;

pub open spec fn bitmap_hi(s: S) -> UInt32;

pub open spec fn bitmap_lo(s: S) -> UInt32;

pub open spec fn IsValidNotificationBitmap(s: S, hi: UInt32, lo: UInt32) -> bool;

pub open spec fn AnyNotificationBoundToOtherSender(s: S, receiver: PartitionId, hi: UInt32, lo: UInt32, sender: PartitionId) -> bool;

pub open spec fn AnyNotificationPending(s: S, receiver: PartitionId, hi: UInt32, lo: UInt32) -> bool;

pub open spec fn CallerAllowedToInvoke(s: S, fid: FunctionId) -> bool;

pub open spec fn SenderPartitionAborted(s: S, sender: PartitionId) -> bool;

pub open spec fn CanSenderSignal(s: S, receiver: PartitionId, sender: PartitionId, n: NotificationId) -> bool;

pub open spec fn SetNotifications(s: S, hi: UInt32, lo: UInt32) -> Set<NotificationId>;

} // verus!
