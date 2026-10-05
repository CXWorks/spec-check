use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt64 = u64;
pub type UInt32 = u32;

pub enum SdeiCommandReturnCode {
    NotSupported,
    InvalidParameters,
    Denied,
    Pending,
    OutOfResource,
}

pub struct SdeiEvent {
    pub handler_state: UInt32,
    pub routing_mode: UInt64,
    pub affinity: UInt64,
}

pub struct S {
    pub sdei_supported: bool,
    pub events: Seq<SdeiEvent>,
}

pub spec const SUCCESS: Result<(), SdeiCommandReturnCode> = Ok(());
pub spec const NOT_SUPPORTED: Result<(), SdeiCommandReturnCode> = Err(SdeiCommandReturnCode::NotSupported);
pub spec const INVALID_PARAMETERS: Result<(), SdeiCommandReturnCode> = Err(SdeiCommandReturnCode::InvalidParameters);
pub spec const DENIED: Result<(), SdeiCommandReturnCode> = Err(SdeiCommandReturnCode::Denied);

pub spec const HANDLER_UNREGISTERED: UInt32 = 0;
pub spec const HANDLER_REGISTERED: UInt32 = 1;
pub spec const HANDLER_ENABLED: UInt32 = 2;

pub spec const RM_ANY: UInt64 = 0;
pub spec const RM_PE: UInt64 = 1;

pub open spec fn SdeiIsSupported(s: S) -> bool;

pub open spec fn ResultEqual(r1: Result<(), SdeiCommandReturnCode>, r2: Result<(), SdeiCommandReturnCode>) -> bool;

pub open spec fn IsValidEventNumber(s: S, event: Int32) -> bool;

pub open spec fn IsSharedEvent(s: S, event: Int32) -> bool;

pub open spec fn IsValidRoutingMode(s: S, routing_mode: UInt64) -> bool;

pub open spec fn RoutingMode(routing_mode: UInt64) -> UInt64;

pub open spec fn IsValidMpidr(s: S, affinity: UInt64) -> bool;

pub open spec fn EventAt(s: S, event: Int32) -> SdeiEvent;

} // verus!
