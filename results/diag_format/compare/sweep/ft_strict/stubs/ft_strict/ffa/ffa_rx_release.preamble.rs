use vstd::prelude::*;

verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: int,
}

pub enum FfaInstance {
    Hypervisor,
    Spmc,
}

pub spec const FFA_RX_RELEASE: UInt32 = 0x84000065u32;

pub spec const ffa_instance: FfaInstance = FfaInstance::Hypervisor;

pub spec const caller: UInt16 = 0u16;

pub spec const FFA_SUCCESS: Result<UInt32, Int32> = Result::Ok(0u32);

pub spec const NOT_SUPPORTED: Result<UInt32, Int32> = Result::Err(-1i32);

pub spec const INVALID_PARAMETERS: Result<UInt32, Int32> = Result::Err(-2i32);

pub spec const DENIED: Result<UInt32, Int32> = Result::Err(-6i32);

pub open spec fn IsImplementedAtInstance(s: S, func_id: UInt32, inst: FfaInstance) -> bool;

pub open spec fn ResultEqual(r: Result<UInt32, Int32>, expected: Result<UInt32, Int32>) -> bool;

pub open spec fn HypervisorRegisteredBufferPairForVm(s: S, vm_id: UInt16) -> bool;

pub open spec fn HasRxBufferOwnership(s: S, owner: UInt16, vm_id: UInt16) -> bool;

} // verus!
