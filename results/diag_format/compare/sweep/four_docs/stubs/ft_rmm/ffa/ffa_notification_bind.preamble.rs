use vstd::prelude::*;

verus! {

pub type UInt16 = u16;
pub type Bool = bool;
pub type Bits32 = u32;
pub type Int32 = i32;
pub type Result = u32;
pub type FuncId = u32;

pub struct S {
    pub dummy: u8,
}

pub struct NotificationBindingInfo {
    pub sender: UInt16,
    pub per_vcpu: Bool,
}

pub const FFA_SUCCESS: Result = 0;
pub const NOT_SUPPORTED: Result = 1;
pub const INVALID_PARAMETERS: Result = 2;
pub const DENIED: Result = 3;
pub const ABORTED: Result = 4;

pub const FFA_NOTIFICATION_BIND: FuncId = 0x8400007F;

pub spec const bitmap: Seq<int> = Seq::empty();

pub open spec fn Exists(b: bool) -> bool;

pub open spec fn ForAll(b: bool) -> bool;

pub open spec fn IsImplementedAtInstance(s: S, func: FuncId) -> bool;

pub open spec fn ResultEqual(r1: Result, r2: Result) -> bool;

pub open spec fn IsValidEndpointId(s: S, id: UInt16) -> bool;

pub open spec fn PerVcpuNotificationsSupported(s: S) -> bool;

pub open spec fn NotificationIsBoundToOtherSender(s: S, receiver_id: UInt16, i: int, sender_id: UInt16) -> bool;

pub open spec fn NotificationIsPending(s: S, receiver_id: UInt16, i: int) -> bool;

pub open spec fn CallerAllowedToInvoke(s: S, func: FuncId) -> bool;

pub open spec fn PartitionHasAborted(s: S, id: UInt16) -> bool;

pub open spec fn NotificationBinding(s: S, receiver_id: UInt16, i: int) -> NotificationBindingInfo;

} // verus!
