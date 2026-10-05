use vstd::prelude::*;
verus! {

pub type UInt64 = u64;
pub type UInt32 = u32;
pub type UInt16 = u16;
pub type Int32 = i32;

pub struct S {
    pub regs: Seq<UInt64>,
}

pub struct PartitionInfoDescriptor {
    pub words: Seq<UInt64>,
}

pub type FfaInstance = u64;

pub spec const ffa_instance: FfaInstance = 0;

pub spec const FFA_SUCCESS64: UInt32 = 0xC4000061;
pub spec const FFA_PARTITION_INFO_GET_REGS: UInt32 = 0xC400008B;

pub spec const NOT_SUPPORTED: Int32 = -1;
pub spec const INVALID_PARAMETERS: Int32 = -2;
pub spec const DENIED: Int32 = -6;
pub spec const RETRY: Int32 = -7;
pub spec const NOT_READY: Int32 = -9;

pub open spec fn ResultEqual<T>(a: T, b: T) -> bool;

pub open spec fn IsValidUuid(s: S, uuid_lo: UInt64, uuid_hi: UInt64) -> bool;

pub open spec fn IsNilUuid(s: S, uuid_lo: UInt64, uuid_hi: UInt64) -> bool;

pub open spec fn IsValidStartIndex(s: S, uuid_lo: UInt64, uuid_hi: UInt64, start_index: UInt16) -> bool;

pub open spec fn IsImplementedAtInstance(s: S, func_id: UInt32, instance: FfaInstance) -> bool;

pub open spec fn CalleeInStateToHandleRequest(s: S) -> bool;

pub open spec fn CalleeReadyToHandleRequest(s: S) -> bool;

pub open spec fn CalleeInfoTag(s: S, uuid_lo: UInt64, uuid_hi: UInt64) -> UInt16;

pub open spec fn PartitionInfoCount(s: S, uuid_lo: UInt64, uuid_hi: UInt64) -> int;

pub open spec fn DescriptorAt(s: S, partition_info: [UInt64; 15], idx: int) -> PartitionInfoDescriptor;

pub open spec fn PartitionInfoEntry(s: S, uuid_lo: UInt64, uuid_hi: UInt64, i: UInt64) -> PartitionInfoDescriptor;

pub open spec fn IsReturnedDescriptorBase(n: UInt64, start_index: UInt16, current_index: UInt16) -> bool;

pub open spec fn IsReturnedDescriptorRegister(n: UInt64, start_index: UInt16, current_index: UInt16) -> bool;

pub open spec fn Reg(s: S, n: int) -> UInt64;

} // verus!
