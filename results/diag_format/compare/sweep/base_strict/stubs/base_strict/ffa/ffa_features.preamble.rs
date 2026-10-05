use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;

pub struct S {
    pub cmd_input_id: UInt32,
    pub cmd_input_props: UInt64,
}

pub spec const FFA_SUCCESS: UInt32 = 0;
pub spec const NOT_SUPPORTED: UInt32 = 1;

pub open spec fn Bits64(value: int, hi: int, lo: int) -> int;

pub open spec fn IsImplementedFfaFunction(id: UInt32) -> bool;

pub open spec fn IsSupportedFrameworkFeature(feature: int) -> bool;

pub open spec fn ResultEqual(a: UInt32, b: UInt32) -> bool;

pub open spec fn iface_props(s: S, lo: int, hi: int) -> int;

pub open spec fn QueriedInterfaceProperties(id: UInt32, props: UInt64) -> int;

} // verus!
