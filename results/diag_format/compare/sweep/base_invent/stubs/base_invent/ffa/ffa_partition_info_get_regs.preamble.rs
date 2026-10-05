use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type UInt32 = u32;

pub type FfaStatusCode = i32;

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Result<T, E> {
    pub open spec fn is_Ok(&self) -> bool;

    pub open spec fn is_Err(&self) -> bool;
}

pub struct S {
    pub partition_info_tag: UInt32,
    pub partition_info_last_index: UInt32,
    pub partition_info_current_index: UInt32,
    pub partition_info_descriptor_size: UInt32,
    pub partition_info_descriptors: Seq<u8>,
}

pub const FFA_ERROR_NOT_SUPPORTED: FfaStatusCode = -1;

pub const FFA_ERROR_INVALID_PARAMETERS: FfaStatusCode = -2;

pub const FFA_ERROR_DENIED: FfaStatusCode = -6;

pub const FFA_ERROR_RETRY: FfaStatusCode = -7;

pub const FFA_ERROR_NOT_READY: FfaStatusCode = -9;

pub open spec fn ResultEqual(result: Result<(), FfaStatusCode>, code: FfaStatusCode) -> bool;

pub open spec fn InvalidUuid(uuid_lo: UInt64, uuid_hi: UInt64) -> bool;

pub open spec fn InvalidStartIndex(start_index: UInt32) -> bool;

pub open spec fn IsPartitionInfoGetRegsSupported(s: S) -> bool;

pub open spec fn IsCalleeReadyToHandleRequest(s: S) -> bool;

} // verus!
