use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub struct S {
    pub drtm_supported: bool,
}

pub const NOT_SUPPORTED: UInt32 = 0xFFFF_FFFFu32;

pub open spec fn DrtmIsSupported(s: S) -> bool;

pub open spec fn ResultEqual(a: UInt32, b: UInt32) -> bool;

pub open spec fn Bits(x: UInt32, hi: nat, lo: nat) -> nat;

pub open spec fn AllNonOptionalDrtmFunctionsImplemented() -> bool;

} // verus!
