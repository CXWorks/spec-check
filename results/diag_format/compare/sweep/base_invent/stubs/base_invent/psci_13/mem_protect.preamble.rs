use vstd::prelude::*;
verus! {

pub const NOT_SUPPORTED: u64 = 0xFFFF_FFFF_FFFF_FFFFu64;

pub struct PsciFeatureSet {
    pub mem_protect: u64,
}

pub struct S {
    pub mem_protect_enabled: bool,
}

pub open spec fn PsciFeatures(s: S) -> PsciFeatureSet;

} // verus!
