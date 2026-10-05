use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt16 = u16;
pub type FfaFunctionId = u32;
pub type VmId = u16;

pub enum FfaInstance {
    NonSecure,
    Secure,
}

pub struct RxBuffer {
    pub owner: u16,
    pub base: u64,
    pub size: u64,
}

pub struct S {
    pub current_vm: u16,
    pub rx_buffers: Map<u16, RxBuffer>,
}

pub const FFA_RX_RELEASE: FfaFunctionId = 0x84000065u32;

pub const FFA_SUCCESS: UInt32 = 0u32;
pub const NOT_SUPPORTED: UInt32 = 1u32;
pub const INVALID_PARAMETERS: UInt32 = 2u32;
pub const DENIED: UInt32 = 3u32;

pub open spec fn IsImplementedAtInstance(func: FfaFunctionId, inst: FfaInstance) -> bool;

pub open spec fn CurrentFfaInstance() -> FfaInstance;

pub open spec fn ResultEqual(result: UInt32, code: UInt32) -> bool;

pub open spec fn vm_id(s: S) -> VmId;

pub open spec fn IsRxTxPairRegisteredByHypervisor(s: S, id: VmId) -> bool;

pub open spec fn TargetRxBuffer(s: S, id: VmId) -> RxBuffer;

pub open spec fn CallerOwnsRxBuffer(s: S, buf: RxBuffer) -> bool;

} // verus!
