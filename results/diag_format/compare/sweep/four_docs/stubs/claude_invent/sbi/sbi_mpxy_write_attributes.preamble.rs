use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type SbiError = i64;

pub struct SbiRet {
    pub error: SbiError,
    pub value: i64,
}

pub struct S {
    pub mpxy_shmem_enabled: bool,
    pub dummy: int,
}

pub const SBI_SUCCESS: SbiError = 0;
pub const SBI_ERR_NOT_SUPPORTED: SbiError = -2;
pub const SBI_ERR_INVALID_PARAM: SbiError = -3;
pub const SBI_ERR_DENIED: SbiError = -4;
pub const SBI_ERR_BAD_RANGE: SbiError = -11;
pub const SBI_ERR_NO_SHMEM: SbiError = -9;

pub open spec fn MpxyShmemEnabled(s: S) -> bool;

pub open spec fn MpxyChannelIsValid(s: S, channel_id: UInt32) -> bool;

pub open spec fn MpxyAttributeIdIsValid(s: S, channel_id: UInt32, attribute_id: UInt32) -> bool;

pub open spec fn MpxyAttributeRangeIsValid(s: S, channel_id: UInt32, base_attribute_id: UInt32, attribute_count: UInt32) -> bool;

pub open spec fn MpxyAttributeRangeHasReadOnly(s: S, channel_id: UInt32, base_attribute_id: UInt32, attribute_count: UInt32) -> bool;

pub open spec fn MpxyChannelAttributesUnchanged(old_s: S, new_s: S, channel_id: UInt32) -> bool;

pub open spec fn MpxyChannelAttribute(s: S, channel_id: UInt32, attribute_id: int) -> u32;

pub open spec fn MpxyShmemReadU32(s: S, offset: int) -> u32;

} // verus!
