use vstd::prelude::*;

verus! {

pub type Int64 = i64;
pub type UInt64 = u64;
pub type UInt32 = u32;

pub struct S {
    pub dummy: u64,
}

pub spec const SUCCESS: Int64 = 0;
pub spec const NOT_SUPPORTED: Int64 = (-1int) as Int64;
pub spec const INVALID_PARAMETERS: Int64 = (-2int) as Int64;
pub spec const DENIED: Int64 = (-3int) as Int64;

pub spec const RM_PE: UInt64 = 1;
pub spec const HANDLER_UNREGISTER_PENDING: UInt64 = 2;

#[allow(non_upper_case_globals)]
pub spec const event: UInt32 = 7;
#[allow(non_upper_case_globals)]
pub spec const entry_point_address: UInt64 = 10;
#[allow(non_upper_case_globals)]
pub spec const flags: UInt64 = 11;
#[allow(non_upper_case_globals)]
pub spec const ep_argument: UInt64 = 12;
#[allow(non_upper_case_globals)]
pub spec const affinity: UInt64 = 13;

pub uninterp spec fn SdeiSupported() -> bool;
pub uninterp spec fn ResultEqual(a: Int64, b: Int64) -> bool;
pub uninterp spec fn IsValidEvent(e: UInt32) -> bool;
pub uninterp spec fn Bits(x: UInt64, hi: int, lo: int) -> UInt64;
pub uninterp spec fn DispatcherDetectsInvalidEntryPoint(addr: UInt64, rm: UInt64) -> bool;
pub uninterp spec fn IsSharedEvent(e: UInt32) -> bool;
pub uninterp spec fn IsValidRoutingMode(rm: UInt64) -> bool;
pub uninterp spec fn IsValidAffinity(a: UInt64) -> bool;
pub uninterp spec fn CallingClient() -> UInt64;
pub uninterp spec fn CallingPe() -> UInt64;
pub uninterp spec fn IsRegisteredByClient(e: UInt32, client: UInt64) -> bool;
pub uninterp spec fn EventHandlerState(e: UInt32) -> UInt64;
pub uninterp spec fn IsRegisteredForClient(e: UInt32, client: UInt64) -> bool;
pub uninterp spec fn IsRegisteredForPe(e: UInt32, pe: UInt64) -> bool;
pub uninterp spec fn EventEnabled(e: UInt32) -> bool;
pub uninterp spec fn EventEntryPoint(e: UInt32) -> UInt64;
pub uninterp spec fn ResolveEntryPoint(addr: UInt64, rel: UInt64) -> UInt64;
pub uninterp spec fn EventArgument(e: UInt32) -> UInt64;
pub uninterp spec fn EventRoutingMode(e: UInt32) -> UInt64;
pub uninterp spec fn EventAffinity(e: UInt32) -> UInt64;

} // verus!
