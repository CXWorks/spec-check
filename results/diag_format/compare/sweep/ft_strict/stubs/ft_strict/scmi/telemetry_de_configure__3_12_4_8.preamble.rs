use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type UInt64 = u64;
pub type DeId = u32;
pub type EventGroupId = u32;

pub struct TelemetryDeConfigureFlags {
    pub reserved: u32,
    pub de_mode: u32,
    pub disable_all: u32,
    pub selector: u32,
}

pub struct S {
    pub dummy: u64,
}

pub const SUCCESS: Int32 = 0;
pub const INVALID_PARAMETERS: Int32 = 1;
pub const IN_USE: Int32 = 2;
pub const OUT_OF_RANGE: Int32 = 3;

pub const RSI_SUCCESS: UInt64 = 0;
pub const result: UInt64 = 1;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;
pub open spec fn IsValidDeId(id: UInt32) -> bool;
pub open spec fn IsValidEventGroupId(id: UInt32) -> bool;
pub open spec fn InUseCheckImplemented() -> bool;
pub open spec fn DeInEventGroup(de: DeId, grp: EventGroupId) -> bool;
pub open spec fn DeIsEnabled(de: DeId) -> bool;
pub open spec fn DeEnabledByEventGroup(de: DeId) -> bool;
pub open spec fn EnabledCountAtPlatformLimit() -> bool;
pub open spec fn EventGroupIsEnabled(grp: EventGroupId) -> bool;
pub open spec fn DeTimestampsEnabled(de: DeId) -> bool;
pub open spec fn ShmtiSupported() -> bool;
pub open spec fn ShmtiUsedFor(identifier: UInt32, selector: u32) -> bool;
pub open spec fn PlatformReturnsShmtiInfo() -> bool;
pub open spec fn ShmtiAllocatedTo(shmti_id: UInt32, identifier: UInt32, selector: u32) -> bool;
pub open spec fn DeLineMetadataOffset(s: S, shmti_id: UInt32, identifier: UInt32) -> UInt32;
pub open spec fn MinDeLineMetadataOffsetInGroup(s: S, shmti_id: UInt32, identifier: UInt32) -> UInt32;
pub open spec fn DeUsesBlockTimestamps(identifier: UInt32) -> bool;
pub open spec fn BlockTimestampLineOffset(s: S, shmti_id: UInt32, identifier: UInt32) -> UInt32;
pub open spec fn IsCompliantBlockTimestampLine(s: S, shmti_id: UInt32, offset: UInt32) -> bool;

} // verus!
