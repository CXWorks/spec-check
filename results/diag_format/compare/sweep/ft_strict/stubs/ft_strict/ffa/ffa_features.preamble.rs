use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub struct S {
    pub dummy: u32,
}

pub const FFA_SUCCESS: UInt32 = 0;
pub const NOT_SUPPORTED: UInt32 = 1;

pub open spec fn IsImplementedFfaFunction(s: S, id: UInt32) -> bool;

pub open spec fn IsSupportedFrameworkFeature(s: S, feature: UInt32) -> bool;

pub open spec fn ResultEqual(result: UInt32, code: UInt32) -> bool;

pub open spec fn QueriedInterfaceProperties(s: S, id: UInt32, input_props: UInt32) -> [UInt32; 2];

} // verus!
