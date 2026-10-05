use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt64 = u64;
pub type PartitionId = u64;

pub struct S {
    pub dummy: u64,
}

pub enum FfaInstance {
    Physical,
    Virtual,
}

pub const FFA_SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const DENIED: Int32 = -6;
pub const ABORTED: Int32 = -8;

pub const ids: UInt64 = 1;
pub const flags: UInt64 = 2;
pub const bitmap: UInt64 = 3;

pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;

pub open spec fn CurrentFfaInstance() -> FfaInstance;

pub open spec fn IsImplementedAtInstance(s: S, inst: FfaInstance) -> bool;

pub open spec fn Bits64(s: S, value: UInt64, hi: UInt64, lo: UInt64) -> UInt64;

pub open spec fn IsRecognizedPartitionId(s: S, id: UInt64) -> bool;

pub open spec fn IsValidNotificationSetFlags(s: S, f: UInt64) -> bool;

pub open spec fn BitmapContainsPerVcpuNotification(s: S, receiver: UInt64, bm: UInt64) -> bool;

pub open spec fn BitmapContainsGlobalNotification(s: S, receiver: UInt64, bm: UInt64) -> bool;

pub open spec fn PerVcpuNotificationsSupported(s: S) -> bool;

pub open spec fn NumSupportedNotifications(s: S) -> UInt64;

pub open spec fn BitmapBit(s: S, bm: UInt64, i: UInt64) -> UInt64;

pub open spec fn SenderMaySignalNotification(s: S, sender: UInt64, receiver: UInt64, i: UInt64) -> bool;

pub open spec fn ReceiverSupportsNotifications(s: S, receiver: UInt64) -> bool;

pub open spec fn PartitionHasAborted(s: S, id: UInt64) -> bool;

pub open spec fn GlobalNotificationSignaled(s: S, receiver: UInt64, i: UInt64) -> bool;

pub open spec fn PerVcpuNotificationSignaled(s: S, receiver: UInt64, vcpu: UInt64, i: UInt64) -> bool;

} // verus!
