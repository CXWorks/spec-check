use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type Int32 = i32;

pub struct FfaResult {
    pub func_id: UInt32,
    pub error_code: Int32,
}

pub struct S {
    pub notification_get_implemented: bool,
    pub caller_allowed: bool,
    pub non_secure_physical_instance: bool,
}

pub const NOT_SUPPORTED: Int32 = -1i32;
pub const INVALID_PARAMETERS: Int32 = -2i32;
pub const DENIED: Int32 = -6i32;

pub open spec fn IsNotificationGetImplemented(s: S) -> bool;

pub open spec fn IsCallerAllowedNotificationGet(s: S) -> bool;

pub open spec fn IsValidPartitionId(s: S, id: UInt32) -> bool;

pub open spec fn IsNonSecurePhysicalInstance(s: S) -> bool;

pub open spec fn FfaResultIsError(r: FfaResult) -> bool;

pub open spec fn FfaResultIsSuccess(r: FfaResult) -> bool;

pub open spec fn FfaResultErrorEqual(r: FfaResult, code: Int32) -> bool;

pub open spec fn PendingSpNotifications(s: S, receiver: UInt32, vcpu: UInt32) -> u64;

pub open spec fn PendingVmNotifications(s: S, receiver: UInt32, vcpu: UInt32) -> u64;

pub open spec fn PendingSpmFrameworkNotifications(s: S, receiver: UInt32, vcpu: UInt32) -> UInt32;

pub open spec fn PendingHypervisorFrameworkNotifications(s: S, receiver: UInt32, vcpu: UInt32) -> UInt32;

} // verus!
