use vstd::prelude::*;
verus! {

pub type UInt64 = int;

pub type SbiCommandReturnCode = int;

pub spec const SBI_SUCCESS: SbiCommandReturnCode = 0;
pub spec const SBI_ERR_INVALID_PARAM: SbiCommandReturnCode = -3;

pub spec const XLEN: int = 64;

pub struct SharedMem {
    pub nacl_sync_hfence_entries: Seq<int>,
}

pub struct S {
    pub shared_memory: SharedMem,
}

pub open spec fn SharedMemory(s: S) -> SharedMem;

} // verus!
