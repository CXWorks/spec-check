use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: i32 = 0;
pub const INVALID_PARAMETERS: i32 = -2;

pub open spec fn IsSkipValid(s: S, skip: UInt32) -> bool;

pub open spec fn AllowedProtocolCount(s: S) -> int;

pub open spec fn PackedProtocolListEqual(s: S, skip: UInt32, num_protocols: UInt32, protocols: Seq<UInt32>) -> bool;

pub open spec fn ProtocolListIsAscending(num_protocols: UInt32, protocols: Seq<UInt32>) -> bool;

pub open spec fn ProtocolListExcludesBase(num_protocols: UInt32, protocols: Seq<UInt32>) -> bool;

} // verus!
