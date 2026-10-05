use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type EndpointId = u16;
pub type FunctionId = u32;

pub struct S {
    pub sender_id: EndpointId,
    pub receiver_id: EndpointId,
    pub per_vcpu: u32,
    pub bitmap: Seq<u8>,
}

pub struct Binding {
    pub sender: EndpointId,
    pub per_vcpu: u32,
}

pub spec const FFA_NOTIFICATION_BIND: FunctionId = 0x8400007F;

pub spec const FFA_SUCCESS: UInt32 = 0;
pub spec const NOT_SUPPORTED: UInt32 = 1;
pub spec const INVALID_PARAMETERS: UInt32 = 2;
pub spec const DENIED: UInt32 = 3;
pub spec const ABORTED: UInt32 = 4;

pub spec const bitmap: Seq<u8>;

pub open spec fn IsImplementedAtInstance(f: FunctionId) -> bool;
pub open spec fn ResultEqual(a: UInt32, b: UInt32) -> bool;
pub open spec fn IsValidEndpointId(id: EndpointId) -> bool;
pub open spec fn sender_id(s: S) -> EndpointId;
pub open spec fn receiver_id(s: S) -> EndpointId;
pub open spec fn per_vcpu(s: S) -> u32;
pub open spec fn PerVcpuNotificationsSupported() -> bool;
pub open spec fn Exists(b: bool) -> bool;
pub open spec fn ForAll(b: bool) -> bool;
pub open spec fn NotificationIsBoundToOtherSender(receiver: EndpointId, i: int, sender: EndpointId) -> bool;
pub open spec fn NotificationIsPending(receiver: EndpointId, i: int) -> bool;
pub open spec fn CallerAllowedToInvoke(f: FunctionId) -> bool;
pub open spec fn PartitionHasAborted(id: EndpointId) -> bool;
pub open spec fn NotificationBinding(receiver: EndpointId, i: int) -> Binding;

} // verus!
