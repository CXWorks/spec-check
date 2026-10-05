use vstd::prelude::*;

verus! {

pub struct S {
    pub sdei_supported: bool,
    pub pe_masked: bool,
    pub other: int,
}

pub const NOT_SUPPORTED: i64 = -1;

pub open spec fn SdeiIsSupported(s: S) -> bool;

pub open spec fn SdeiPeIsMasked(s: S) -> bool;

pub open spec fn SdeiPeMaskOnlyChanged(old_s: S, new_s: S) -> bool;

} // verus!
