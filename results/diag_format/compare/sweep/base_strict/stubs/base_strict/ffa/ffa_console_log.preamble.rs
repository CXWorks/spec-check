use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub struct S {
    pub console_output: int,
}

pub spec const FFA_CONSOLE_LOG: UInt32 = 0x8400008A;

pub spec const FFA_SUCCESS: UInt32 = 0x84000061;
pub spec const FFA_ERROR_NOT_SUPPORTED: UInt32 = 0xFFFFFFFF;
pub spec const FFA_ERROR_INVALID_PARAMETERS: UInt32 = 0xFFFFFFFE;
pub spec const FFA_ERROR_RETRY: UInt32 = 0xFFFFFFF7;

pub spec const fid: UInt32 = 0xC400008A;

pub spec const logged_count: UInt32 = 7;

pub open spec fn IsImplementedAtInstance(s: S, func_id: UInt32) -> bool;

pub open spec fn ResultEqual(result: UInt32, expected: UInt32) -> bool;

pub open spec fn IsSmc32Convention(s: S, func_id: UInt32) -> bool;

pub open spec fn IsSmc64Convention(s: S, func_id: UInt32) -> bool;

pub open spec fn AllCharactersLogged(s: S, characters: [UInt32; 6], char_count: UInt32) -> bool;

pub open spec fn CharactersLoggedToConsoleInFiniteTime(s: S, characters: [UInt32; 6], char_count: UInt32) -> bool;

} // verus!
