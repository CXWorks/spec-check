use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt64 = u64;

pub enum SdeiCommandReturnCode {
    NotSupported,
    InvalidParameters,
    Denied,
    Pending,
    OutOfResource,
}

pub struct S {
    pub sdei_supported: bool,
    pub dummy: u64,
}

pub spec const SUCCESS: Result<(), SdeiCommandReturnCode> = Ok(());
pub spec const NOT_SUPPORTED: Result<(), SdeiCommandReturnCode> = Err(SdeiCommandReturnCode::NotSupported);
pub spec const INVALID_PARAMETERS: Result<(), SdeiCommandReturnCode> = Err(SdeiCommandReturnCode::InvalidParameters);
pub spec const DENIED: Result<(), SdeiCommandReturnCode> = Err(SdeiCommandReturnCode::Denied);

pub spec const RM_ANY: UInt64 = 0;
pub spec const RM_PE: UInt64 = 1;

pub spec const HANDLER_UNREGISTERED: u64 = 0;
pub spec const HANDLER_REGISTERED: u64 = 1;
pub spec const HANDLER_ENABLED: u64 = 2;

pub open spec fn SdeiIsSupported(s: S) -> bool;
pub open spec fn IsValidEventNumber(s: S, event: Int32) -> bool;
pub open spec fn IsSharedEvent(s: S, event: Int32) -> bool;
pub open spec fn IsValidRoutingMode(s: S, routing_mode: UInt64) -> bool;
pub open spec fn IsValidAffinity(s: S, affinity: UInt64) -> bool;
pub open spec fn Bits(x: UInt64, hi: int, lo: int) -> UInt64;
pub open spec fn EventHandlerState(s: S, event: Int32) -> u64;
pub open spec fn EventRoutingMode(s: S, event: Int32) -> UInt64;
pub open spec fn EventRoutingAffinity(s: S, event: Int32) -> UInt64;
pub open spec fn ResultEqual(a: Result<(), SdeiCommandReturnCode>, b: Result<(), SdeiCommandReturnCode>) -> bool;

} // verus!
