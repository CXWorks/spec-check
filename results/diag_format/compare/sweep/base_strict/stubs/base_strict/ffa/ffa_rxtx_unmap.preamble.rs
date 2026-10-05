use vstd::prelude::*;
verus! {

pub type FfaFunctionId = u32;

pub type FfaReturnCode = i32;

pub type EndpointId = u16;

pub struct Endpoint {
    pub id: EndpointId,
}

pub struct FfaInstance {
    pub id: u8,
}

pub struct S {
    pub dummy: u64,
}

pub spec const FFA_RXTX_UNMAP: FfaFunctionId = 0x84000067u32;

pub spec const FFA_SUCCESS: FfaReturnCode = 0i32;
pub spec const NOT_SUPPORTED: FfaReturnCode = -1i32;
pub spec const INVALID_PARAMETERS: FfaReturnCode = -2i32;

pub open spec fn IsImplementedAtInstance(func: FfaFunctionId, inst: FfaInstance) -> bool;

pub open spec fn CurrentFfaInstance() -> FfaInstance;

pub open spec fn ResultEqual(result: u32, code: FfaReturnCode) -> bool;

pub open spec fn CallerEndpoint(id: u16) -> Endpoint;

pub open spec fn IsRxTxBufferPairRegistered(ep: Endpoint) -> bool;

pub open spec fn IsRxTxBufferPairMappedInCalleeRegime(ep: Endpoint) -> bool;

pub open spec fn RxTxBufferPairMappedInCalleeRegime(s: S, ep: Endpoint) -> bool;

} // verus!
