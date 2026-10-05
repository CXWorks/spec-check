use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type UInt16 = u16;
pub type FunctionId = u32;
pub type EndpointId = u16;
pub type VmId = u16;

pub struct S {
    pub caller_id: EndpointId,
    pub current_vm_id: VmId,
    pub state_tag: u64,
}

pub enum RxOwner {
    Producer,
    Consumer,
    Hypervisor,
    None,
}

pub const FFA_RX_RELEASE: FunctionId = 0x8400_0065;

pub const FFA_SUCCESS: UInt32 = 0;
pub const NOT_SUPPORTED: UInt32 = 1;
pub const INVALID_PARAMETERS: UInt32 = 2;
pub const DENIED: UInt32 = 3;

pub open spec fn IsImplementedAtInstance(s: S, fid: FunctionId) -> bool;

pub open spec fn ResultEqual(result: UInt32, code: UInt32) -> bool;

pub open spec fn HypervisorRegisteredBufferPairForVm(s: S, vm: VmId) -> bool;

pub open spec fn vm_id(s: S) -> VmId;

pub open spec fn caller(s: S) -> EndpointId;

pub open spec fn HasRxBufferOwnership(s: S, ep: EndpointId, vm: VmId) -> bool;

pub open spec fn RxBufferOwnership(s: S, ep: EndpointId, vm: VmId) -> RxOwner;

} // verus!
