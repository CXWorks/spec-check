use vstd::prelude::*;

verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;
pub type UInt64 = u64;
pub type UInt = u64;

pub type FfaStatusCode = i32;
pub type FunctionId = u32;
pub type Instance = u8;
pub type NotificationClass = u8;

pub struct S {
    pub dummy: int,
}

pub spec const NOT_SUPPORTED: FfaStatusCode = -1;
pub spec const INVALID_PARAMETERS: FfaStatusCode = -2;
pub spec const DENIED: FfaStatusCode = -6;

pub spec const FFA_SUCCESS: Result<UInt32, FfaStatusCode> = Ok(0u32);

pub spec const FFA_NOTIFICATION_GET: FunctionId = 0x84000082;

pub spec const NS_PHYSICAL: Instance = 0;
pub spec const S_PHYSICAL: Instance = 1;
pub spec const NS_VIRTUAL: Instance = 2;
pub spec const S_VIRTUAL: Instance = 3;

pub spec const SP: NotificationClass = 0;
pub spec const VM: NotificationClass = 1;
pub spec const SPM_FRAMEWORK: NotificationClass = 2;
pub spec const HYP_FRAMEWORK: NotificationClass = 3;

pub open spec fn IsImplementedAtInstance(s: S, func: FunctionId, inst: Instance) -> bool;

pub open spec fn CurrentInstance(s: S) -> Instance;

pub open spec fn CallerMayInvoke(s: S, func: FunctionId) -> bool;

pub open spec fn IsRecognizedPartitionId(s: S, id: UInt16) -> bool;

pub open spec fn ResultEqual(result: Result<UInt32, FfaStatusCode>, code: FfaStatusCode) -> bool;

pub open spec fn PendingNotifications(s: S, receiver_id: UInt16, receiver_vcpu_id: UInt16, class: NotificationClass) -> UInt64;

} // verus!
