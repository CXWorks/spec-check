use vstd::prelude::*;

verus! {

#[allow(non_camel_case_types)]
pub type unsigned_long = u64;

pub type SbiCommandReturnCode = i64;

pub const SBI_SUCCESS: SbiCommandReturnCode = 0;

pub struct S {
    pub shmem_base: u64,
    pub shmem_size: u64,
    pub shmem_disabled: bool,
    pub shmem_setup_mode: u64,
}

pub open spec fn IsAllOnes(x: unsigned_long) -> bool;

pub open spec fn SharedMemoryBase(s: S) -> u64;

pub open spec fn SharedMemorySize(s: S) -> u64;

pub open spec fn SharedMemoryDisabled(s: S) -> bool;

pub open spec fn SharedMemorySetupMode(s: S) -> u64;

pub open spec fn ConcatPhysAddr(s: S, hi: unsigned_long, lo: unsigned_long) -> u64;

pub open spec fn MpxyGetShmemSize(s: S) -> u64;

pub open spec fn Bits(s: S, value: unsigned_long, hi: int, lo: int) -> u64;

} // verus!
