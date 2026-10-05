use vstd::prelude::*;

verus! {

pub type Int64 = i64;
pub type UInt64 = u64;

pub struct S {
    pub dummy: u64,
}

pub spec const SUCCESS: Int64 = 0;
pub spec const NOT_SUPPORTED: Int64 = (-1int) as i64;
pub spec const INVALID_PARAMETERS: Int64 = (-2int) as i64;
pub spec const DENIED: Int64 = (-3int) as i64;

pub spec const RM_PE: UInt64 = 1;
pub spec const RM_ANY: UInt64 = 0;

pub spec const HANDLER_REGISTERED: UInt64 = 1;
pub spec const HANDLER_UNREGISTERED: UInt64 = 0;

pub spec const event: UInt64 = 1000;
pub spec const routing_mode: UInt64 = 2000;
pub spec const affinity: UInt64 = 3000;

pub open spec fn SdeiIsSupported() -> bool;

pub open spec fn ResultEqual(result: Int64, code: Int64) -> bool;

pub open spec fn IsValidEventNumber(ev: UInt64) -> bool;

pub open spec fn IsSharedEvent(ev: UInt64) -> bool;

pub open spec fn IsValidRoutingMode(mode: UInt64) -> bool;

pub open spec fn Bits(x: UInt64, hi: int, lo: int) -> UInt64;

pub open spec fn IsValidAffinity(aff: UInt64) -> bool;

pub open spec fn EventHandlerState(ev: UInt64) -> UInt64;

pub open spec fn EventRoutingMode(ev: UInt64) -> UInt64;

pub open spec fn EventRoutingAffinity(ev: UInt64) -> UInt64;

} // verus!
