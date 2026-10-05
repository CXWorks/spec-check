use vstd::prelude::*;
verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;
pub type Int32 = i32;
pub type FfaFunctionId = u32;
pub type EndpointId = u16;

pub enum FfaInstance {
    NonSecurePhysical,
    NonSecureVirtual,
    SecurePhysical,
    SecureVirtual,
}

pub struct S {
    pub dummy: nat,
}

pub const FFA_RXTX_UNMAP: FfaFunctionId = 0x84000067;

pub const FFA_SUCCESS: UInt32 = 0;
pub const NOT_SUPPORTED: UInt32 = 1;
pub const INVALID_PARAMETERS: UInt32 = 2;

pub open spec fn IsImplementedAtInstance(s: S, func: FfaFunctionId, inst: FfaInstance) -> bool;

pub open spec fn CurrentFfaInstance(s: S) -> FfaInstance;

pub open spec fn ResultEqual(result: UInt32, expected: UInt32) -> bool;

pub open spec fn CallerEndpoint(s: S, id: UInt16) -> EndpointId;

pub open spec fn IsRxTxBufferPairRegistered(s: S, ep: EndpointId) -> bool;

pub open spec fn IsRxTxBufferPairMappedInCalleeRegime(s: S, ep: EndpointId) -> bool;

} // verus!
