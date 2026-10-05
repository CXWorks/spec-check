use vstd::prelude::*;

verus! {

pub struct S {
    pub dummy: u32,
}

pub const FFA_SUCCESS: u32 = 0x84000061;
pub const FFA_ERROR: u32 = 0x84000060;
pub const NOT_SUPPORTED: u32 = 0xFFFFFFFF;
pub const INVALID_PARAMETERS: u32 = 0xFFFFFFFE;
pub const NO_MEMORY: u32 = 0xFFFFFFFD;
pub const BUSY: u32 = 0xFFFFFFFC;
pub const INTERRUPTED: u32 = 0xFFFFFFFB;
pub const DENIED: u32 = 0xFFFFFFFA;
pub const RETRY: u32 = 0xFFFFFFF9;
pub const ABORTED: u32 = 0xFFFFFFF8;
pub const NO_DATA: u32 = 0xFFFFFFF7;

} // verus!
