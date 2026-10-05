use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type Int32 = i32;

pub type Flags = u32;

pub struct S {
    pub de_enabled: Map<UInt32, bool>,
    pub de_timestamped: Map<UInt32, bool>,
    pub group_des: Map<UInt32, Set<UInt32>>,
    pub enabled_count: nat,
}

pub const SUCCESS: Int32 = 0;

pub const INVALID_PARAMETERS: Int32 = 1;

pub const IN_USE: Int32 = 2;

pub const OUT_OF_RANGE: Int32 = 3;

pub const result: Int32 = 4;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn IsValidDeOrGroup(s: S, identifier: UInt32, is_group: UInt32) -> bool;

pub open spec fn GroupHasEnabledDe(s: S, identifier: UInt32) -> bool;

pub open spec fn DeEnabledViaGroup(s: S, identifier: UInt32) -> bool;

pub open spec fn EnabledLimitReached(s: S) -> bool;

pub open spec fn DeEnabled(s: S, de: UInt32) -> bool;

pub open spec fn GroupEnabled(g: UInt32) -> bool;

pub open spec fn DeTimestamped(s: S, de: UInt32) -> bool;

pub open spec fn GroupDes(s: S, identifier: UInt32) -> Set<UInt32>;

pub open spec fn EnablingDeOrGroup(s: S, flags: Flags) -> bool;

pub open spec fn ShmtiSupported(s: S) -> bool;

pub open spec fn ShmtiUsedFor(s: S, identifier: UInt32) -> bool;

pub open spec fn ShmtiInfoReturnSupported(s: S) -> bool;

pub open spec fn ShmtiOf(s: S, identifier: UInt32) -> UInt32;

pub open spec fn DeLineMetadataOffset(s: S, identifier: UInt32) -> UInt32;

pub open spec fn TimestampsEnabled(s: S, identifier: UInt32) -> bool;

pub open spec fn UsesBlockTimestamps(s: S, identifier: UInt32) -> bool;

pub open spec fn BlockTimestampLineOffset(s: S, identifier: UInt32) -> UInt32;

} // verus!
