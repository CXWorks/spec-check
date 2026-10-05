use vstd::prelude::*;

verus! {

pub struct S {
    pub x0: u64,
    pub x1: u64,
    pub x2: u64,
    pub x3: u64,
}

pub const INVALID_PARAMETERS: i32 = -2;
pub const NOT_SUPPORTED: i32 = -1;
pub const DENIED: i32 = -6;
pub const RETRY: i32 = -7;
pub const NOT_READY: i32 = -9;

pub const FFA_SUCCESS64: u32 = 0xC4000061;
pub const FFA_PARTITION_INFO_GET_REGS: u32 = 0xC400008B;

pub open spec fn ResultEqual<T>(a: T, b: T) -> bool;

pub open spec fn uuid_lo(s: S) -> u64;
pub open spec fn uuid_hi(s: S) -> u64;
pub open spec fn start_index(s: S) -> u64;
pub open spec fn tag(s: S) -> u16;
pub open spec fn ffa_instance(s: S) -> u32;

pub open spec fn last_index(s: S) -> u64;
pub open spec fn current_index(s: S) -> u64;
pub open spec fn info_tag(s: S) -> u16;
pub open spec fn desc_size(s: S) -> u64;

pub open spec fn CalleeInfoTag(s: S, uuid_lo: u64, uuid_hi: u64) -> u16;
pub open spec fn IsValidUuid(s: S, uuid_lo: u64, uuid_hi: u64) -> bool;
pub open spec fn IsValidStartIndex(s: S, uuid_lo: u64, uuid_hi: u64, start_index: u64) -> bool;
pub open spec fn IsImplementedAtInstance(s: S, fid: u32, instance: u32) -> bool;
pub open spec fn CalleeInStateToHandleRequest(s: S) -> bool;
pub open spec fn CalleeReadyToHandleRequest(s: S) -> bool;
pub open spec fn PartitionInfoCount(s: S, uuid_lo: u64, uuid_hi: u64) -> int;
pub open spec fn DescriptorAt(partition_info: [u64; 15], i: int) -> u64;
pub open spec fn PartitionInfoEntry(s: S, uuid_lo: u64, uuid_hi: u64, i: u64) -> u64;
pub open spec fn IsNilUuid(s: S, uuid_lo: u64, uuid_hi: u64) -> bool;
pub open spec fn IsReturnedDescriptorBase(n: u64, start_index: u64, current_index: u64) -> bool;
pub open spec fn IsReturnedDescriptorRegister(n: u64, start_index: u64, current_index: u64) -> bool;
pub open spec fn Reg<T>(n: T) -> u64;

} // verus!
