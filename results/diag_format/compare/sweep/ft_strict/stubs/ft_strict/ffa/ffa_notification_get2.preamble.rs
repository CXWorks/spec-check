use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type Int32 = i32;

pub type FfaStatusCode = i32;
pub type FfaFunctionId = u32;
pub type FfaInstance = u8;
pub type PartitionId = u16;

pub struct S {
    pub dummy: int,
}

pub spec const NOT_SUPPORTED: FfaStatusCode = (-1) as i32;
pub spec const INVALID_PARAMETERS: FfaStatusCode = (-2) as i32;
pub spec const DENIED: FfaStatusCode = (-6) as i32;

pub spec const FFA_SUCCESS64: Result<(), FfaStatusCode> = Ok(());

pub spec const FFA_NOTIFICATION_GET2: FfaFunctionId = 0xC4000099;

pub spec const NS_PHYSICAL: FfaInstance = 0;

pub uninterp spec fn IsImplementedAtInstance(s: S, func: FfaFunctionId, inst: FfaInstance) -> bool;

pub uninterp spec fn CurrentInstance(s: S) -> FfaInstance;

pub uninterp spec fn ResultEqual(result: Result<(), FfaStatusCode>, code: FfaStatusCode) -> bool;

pub uninterp spec fn CallerAllowedToInvoke(s: S, caller: PartitionId, func: FfaFunctionId) -> bool;

pub uninterp spec fn Caller(s: S) -> PartitionId;

pub uninterp spec fn IsRecognizedPartitionId(s: S, id: int) -> bool;

pub uninterp spec fn Bits<T>(x: T, hi: int, lo: int) -> int;

pub uninterp spec fn ExceedsSupportedNotifications(s: S, bitmask: [UInt64; 6]) -> bool;

pub uninterp spec fn EmptyNotificationBitmapSpecified(s: S, flags: UInt64, receiver_id: UInt32) -> bool;

pub uninterp spec fn RetrievedNotifications<T>(s: S, pending: T, mask: T) -> T;

pub uninterp spec fn PendingSpNotifications(s: S, receiver_id: UInt32) -> [UInt64; 6];

pub uninterp spec fn PendingVmNotifications(s: S, receiver_id: UInt32) -> [UInt64; 6];

pub uninterp spec fn PendingSpmFrameworkNotifications(s: S, receiver_id: UInt32) -> UInt64;

pub uninterp spec fn PendingHypFrameworkNotifications(s: S, receiver_id: UInt32) -> UInt64;

pub uninterp spec fn MaskedNotificationsRemainInCurrentState(s: S, receiver_id: UInt32, sp_bitmask: [UInt64; 6], vm_bitmask: [UInt64; 6], spmc_bitmap: UInt64, hyp_bitmap: UInt64) -> bool;

} // verus!
