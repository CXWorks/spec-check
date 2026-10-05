use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt64 = u64;
pub type DeId = u32;
pub type EventGroupId = u32;

pub struct S {
    pub dummy: u64,
}

pub struct Flags {
    pub disable_all: u32,
    pub selector: u32,
    pub de_mode: u32,
    pub reserved: u32,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const INVALID_PARAMETERS: Int32 = 1;
pub spec const IN_USE: Int32 = 2;
pub spec const OUT_OF_RANGE: Int32 = 3;

pub spec const flags: Flags = vstd::pervasive::arbitrary();
pub spec const identifier: u32 = vstd::pervasive::arbitrary();
pub spec const SHMTI_id: u32 = vstd::pervasive::arbitrary();
pub spec const SHMTI_de_offset: u64 = vstd::pervasive::arbitrary();
pub spec const blk_ts_offset: u64 = vstd::pervasive::arbitrary();

pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;

pub open spec fn DeIsEnabled(de: DeId) -> bool;
pub open spec fn EventGroupIsEnabled(grp: EventGroupId) -> bool;
pub open spec fn DeTimestampsEnabled(de: DeId) -> bool;
pub open spec fn DeInEventGroup(de: DeId, grp: EventGroupId) -> bool;
pub open spec fn DeEnabledByEventGroup(de: DeId) -> bool;
pub open spec fn DeUsesBlockTimestamps(id: u32) -> bool;

pub open spec fn IsValidDeId(id: u32) -> bool;
pub open spec fn IsValidEventGroupId(id: u32) -> bool;
pub open spec fn InUseCheckImplemented() -> bool;
pub open spec fn EnabledCountAtPlatformLimit() -> bool;

pub open spec fn ShmtiSupported() -> bool;
pub open spec fn ShmtiUsedFor(id: u32, selector: u32) -> bool;
pub open spec fn PlatformReturnsShmtiInfo() -> bool;
pub open spec fn ShmtiAllocatedTo(shmti_id: u32, id: u32, selector: u32) -> bool;
pub open spec fn DeLineMetadataOffset(shmti_id: u32, id: u32) -> u64;
pub open spec fn MinDeLineMetadataOffsetInGroup(shmti_id: u32, grp: u32) -> u64;
pub open spec fn BlockTimestampLineOffset(shmti_id: u32, id: u32) -> u64;
pub open spec fn IsCompliantBlockTimestampLine(shmti_id: u32, offset: u64) -> bool;

pub open spec fn flags_reserved_pre(s: S) -> bool;
pub open spec fn de_mode_reserved_pre(s: S) -> bool;
pub open spec fn disable_all_mode_pre(s: S) -> bool;
pub open spec fn de_identifier_pre(s: S) -> bool;
pub open spec fn group_identifier_pre(s: S) -> bool;
pub open spec fn group_de_in_use_pre(s: S) -> bool;
pub open spec fn de_group_in_use_pre(s: S) -> bool;
pub open spec fn enable_limit_pre(s: S) -> bool;

} // verus!
