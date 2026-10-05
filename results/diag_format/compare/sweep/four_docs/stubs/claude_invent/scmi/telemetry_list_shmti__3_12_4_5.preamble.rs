use vstd::prelude::*;

verus! {

pub type ScmiStatus = i32;

pub const SUCCESS: ScmiStatus = 0;
pub const NOT_SUPPORTED: ScmiStatus = -1;
pub const NOT_FOUND: ScmiStatus = -4;

pub struct S {
    pub shmti_supported: bool,
    pub shmti_count: nat,
}

pub open spec fn IsShmtiSupported(s: S) -> bool;

pub open spec fn ShmtiCount(s: S) -> int;

pub open spec fn ShmtiDescAt(s: S, idx: int) -> (u32, u32, u32, u32, u32);

pub open spec fn AddrInAgentMemoryMap(s: S, addr: int) -> bool;

} // verus!
