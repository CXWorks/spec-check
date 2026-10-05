use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct CmdInput {
    pub words: Seq<UInt32>,
}

pub struct S {
    pub cmd_input: CmdInput,
}

pub const SUCCESS: Int32 = 0;
pub const INVALID_PARAMETERS: Int32 = 1;
pub const IN_USE: Int32 = 2;
pub const OUT_OF_RANGE: Int32 = 3;

pub open spec fn PayloadWord0(cmd_input: CmdInput) -> UInt32;
pub open spec fn PayloadWord1(cmd_input: CmdInput) -> UInt32;
pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;
pub open spec fn IsValidDeOrGroup(identifier: UInt32, selector: UInt32) -> bool;
pub open spec fn GroupHasEnabledDe(group: UInt32) -> bool;
pub open spec fn DeEnabledViaGroup(de: UInt32) -> bool;
pub open spec fn EnabledLimitReached() -> bool;
pub open spec fn DeEnabled(de: UInt32) -> bool;
pub open spec fn GroupEnabled(g: UInt32) -> bool;
pub open spec fn DeTimestamped(de: UInt32) -> bool;
pub open spec fn GroupDes(group: UInt32) -> Set<UInt32>;
pub open spec fn EnablingDeOrGroup(flags: UInt32) -> bool;
pub open spec fn ShmtiSupported() -> bool;
pub open spec fn ShmtiUsedFor(identifier: UInt32) -> bool;
pub open spec fn ShmtiInfoReturnSupported() -> bool;
pub open spec fn ShmtiOf(identifier: UInt32) -> UInt32;
pub open spec fn DeLineMetadataOffset(identifier: UInt32) -> UInt32;
pub open spec fn TimestampsEnabled(identifier: UInt32) -> bool;
pub open spec fn UsesBlockTimestamps(identifier: UInt32) -> bool;
pub open spec fn BlockTimestampLineOffset(identifier: UInt32) -> UInt32;

} // verus!
