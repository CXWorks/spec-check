use vstd::prelude::*;

verus! {

pub type UInt8 = u8;
pub type UInt32 = u32;
pub type Int32 = i32;

pub enum FFAReturnCode {
    Success,
}

pub struct S {
    pub dummy: int,
}

pub spec const FFA_SUCCESS: Result<FFAReturnCode, Int32> = Result::Ok(FFAReturnCode::Success);
pub spec const NOT_SUPPORTED: Result<FFAReturnCode, Int32> = Result::Err(-1i32);
pub spec const INVALID_PARAMETERS: Result<FFAReturnCode, Int32> = Result::Err(-2i32);
pub spec const RETRY: Result<FFAReturnCode, Int32> = Result::Err(-7i32);

pub spec const FFA_CONSOLE_LOG: UInt32 = 0x8400008Au32;

pub open spec fn CharCount(char_count: UInt8) -> int;

pub open spec fn ResultEqual(a: Result<FFAReturnCode, Int32>, b: Result<FFAReturnCode, Int32>) -> bool;

pub open spec fn IsSmc32Convention(x: int) -> bool;

pub open spec fn IsSmc64Convention(x: int) -> bool;

pub open spec fn IsImplementedAtInstance(func_id: UInt32) -> bool;

pub open spec fn AllCharactersLogged(char_list: [UInt32; 6], char_count: UInt8) -> bool;

pub open spec fn NumCharactersLogged(char_list: [UInt32; 6], char_count: UInt8) -> UInt32;

} // verus!
