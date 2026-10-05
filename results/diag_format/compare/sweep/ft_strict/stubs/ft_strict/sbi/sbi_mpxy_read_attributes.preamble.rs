use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt = nat;

pub type SbiCommandReturnCode = i64;

pub const SBI_SUCCESS: SbiCommandReturnCode = 0;
pub const SBI_ERR_FAILED: SbiCommandReturnCode = -1;
pub const SBI_ERR_NOT_SUPPORTED: SbiCommandReturnCode = -2;
pub const SBI_ERR_INVALID_PARAM: SbiCommandReturnCode = -3;

pub struct S {
    pub dummy: nat,
}

pub open spec fn ChannelAttributesRead(s: S, channel_id: UInt32, base_attribute_id: UInt32, attribute_count: UInt32) -> bool;

} // verus!
