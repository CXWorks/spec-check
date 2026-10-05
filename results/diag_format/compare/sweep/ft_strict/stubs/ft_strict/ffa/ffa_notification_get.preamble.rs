use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub type FfaFunctionId = u32;

pub type InstanceId = u32;

pub type EndpointId = u32;

pub enum FfaStatusCode {
    NotSupported,
    InvalidParameters,
    NoMemory,
    Busy,
    Interrupted,
    Denied,
    Retry,
    Aborted,
    NoData,
}

pub struct S {
    pub dummy: u64,
}

pub spec const FFA_NOTIFICATION_GET: FfaFunctionId = 0x84000082;

pub spec const FFA_SUCCESS: Result<UInt32, FfaStatusCode> = Ok(0u32);

pub spec const NOT_SUPPORTED: Result<UInt32, FfaStatusCode> = Err(FfaStatusCode::NotSupported);

pub spec const DENIED: Result<UInt32, FfaStatusCode> = Err(FfaStatusCode::Denied);

pub spec const INVALID_PARAMETERS: Result<UInt32, FfaStatusCode> = Err(FfaStatusCode::InvalidParameters);

pub open spec fn IsImplementedAtInstance(s: S, fid: FfaFunctionId, inst: InstanceId) -> bool;

pub open spec fn CurrentInstance(s: S) -> InstanceId;

pub open spec fn ResultEqual(r: Result<UInt32, FfaStatusCode>, expected: Result<UInt32, FfaStatusCode>) -> bool;

pub open spec fn IsCallerAllowedToInvoke(s: S, caller: EndpointId, fid: FfaFunctionId) -> bool;

pub open spec fn Caller() -> EndpointId;

pub open spec fn IsRecognizedPartitionId(s: S, id: UInt32) -> bool;

pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt32;

pub open spec fn IsNonSecurePhysicalInstance(s: S) -> bool;

pub open spec fn PendingSpNotifications(s: S, id: UInt32, vcpu: UInt32) -> UInt32;

pub open spec fn PendingVmNotifications(s: S, id: UInt32, vcpu: UInt32) -> UInt32;

pub open spec fn PendingSpmFrameworkNotifications(s: S, id: UInt32, vcpu: UInt32) -> UInt32;

pub open spec fn PendingHypervisorFrameworkNotifications(s: S, id: UInt32, vcpu: UInt32) -> UInt32;

} // verus!
