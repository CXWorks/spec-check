use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type Int32 = i32;

pub enum FFAStatus {
    Success,
}

pub struct S {
    pub dummy: int,
}

pub const FFA_CONSOLE_LOG: UInt32 = 0x8400008A;

pub const FFA_SUCCESS: Result<FFAStatus, Int32> = Ok(FFAStatus::Success);

pub const NOT_SUPPORTED: Result<FFAStatus, Int32> = Err(-1i32);

pub const INVALID_PARAMETERS: Result<FFAStatus, Int32> = Err(-2i32);

pub open spec fn RETRY(n: int) -> Result<FFAStatus, Int32>;

pub open spec fn ResultEqual(a: Result<FFAStatus, Int32>, b: Result<FFAStatus, Int32>) -> bool;

pub open spec fn IsImplementedAtInstance(s: S, func_id: UInt32) -> bool;

pub open spec fn IsSmc32Convention(s: S, func_id: UInt32) -> bool;

pub open spec fn IsSmc64Convention(s: S, func_id: UInt32) -> bool;

pub open spec fn AllCharactersLogged(s: S, characters: [UInt32; 6], count: int) -> bool;

pub open spec fn CharactersLoggedToConsoleInFiniteTime(s: S, characters: [UInt32; 6], count: int) -> bool;

} // verus!
