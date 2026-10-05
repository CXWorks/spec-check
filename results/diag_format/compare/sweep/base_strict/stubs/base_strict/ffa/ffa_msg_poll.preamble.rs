use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type EndpointId = u16;

pub type InstanceId = u32;

pub struct S {
    pub state_id: nat,
}

pub spec const RETRY: UInt32 = 0xFFFF_FFF9;
pub spec const DENIED: UInt32 = 0xFFFF_FFFA;
pub spec const NOT_SUPPORTED: UInt32 = 0xFFFF_FFFF;
pub spec const FFA_MSG_SEND: UInt32 = 0x8400_006E;
pub spec const FFA_MSG_POLL: UInt32 = 0x8400_006A;

pub spec const caller: EndpointId = 1;
pub spec const callee: EndpointId = 2;

pub spec const ffa_instance: InstanceId = 0;

pub open spec fn MessageAvailableInRxBuffer(s: S, endpoint: EndpointId) -> bool;

pub open spec fn CalleeInStateToHandleRequest(s: S, endpoint: EndpointId) -> bool;

pub open spec fn IsImplementedAtInstance(s: S, func_id: UInt32, instance: InstanceId) -> bool;

pub open spec fn ResultEqual(result: UInt32, expected: UInt32) -> bool;

} // verus!
