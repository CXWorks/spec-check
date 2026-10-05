use vstd::prelude::*;
verus! {

pub struct S {
    pub w0: u32,
    pub w1: u32,
    pub w2: u32,
    pub w3: u32,
    pub w4: u32,
    pub w5: u32,
    pub w6: u32,
    pub w7: u32,
    pub processing_direct_request: bool,
}

pub const FFA_SUCCESS: u32 = 0x84000061u32;

pub open spec fn IsProcessingDirectRequest(s: S) -> bool;

pub open spec fn ResultIsError(r: u32) -> bool;

pub open spec fn w1_is_mbz(s: S) -> bool;

pub open spec fn w2_is_mbz(s: S) -> bool;

pub open spec fn w3_is_mbz(s: S) -> bool;

pub open spec fn w4_is_mbz(s: S) -> bool;

pub open spec fn w5_is_mbz(s: S) -> bool;

pub open spec fn w6_is_mbz(s: S) -> bool;

pub open spec fn w7_is_mbz(s: S) -> bool;

} // verus!
