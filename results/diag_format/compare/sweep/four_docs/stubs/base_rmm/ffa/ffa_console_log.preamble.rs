use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt64 = u64;

pub struct S {
    pub fid: UInt32,
    pub char_count: UInt64,
    pub char_list: Seq<u8>,
}

pub const FFA_SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const RETRY: Int32 = -7;

pub const FFA_CONSOLE_LOG: UInt32 = 0x8400008A;

pub open spec fn CharCount(char_count: UInt64) -> int;

pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;

pub open spec fn IsSmc32Convention(fid: UInt32) -> bool;

pub open spec fn IsSmc64Convention(fid: UInt32) -> bool;

pub open spec fn IsImplementedAtInstance(s: S, fid: UInt32) -> bool;

pub open spec fn AllCharactersLogged(char_list: Seq<u8>, char_count: UInt64) -> bool;

pub open spec fn NumCharactersLogged(char_list: Seq<u8>, char_count: UInt64) -> UInt32;

pub open spec fn CharactersLoggedToConsole(char_list: Seq<u8>, char_count: UInt64) -> bool;

} // verus!
