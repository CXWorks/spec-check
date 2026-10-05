use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;

pub struct S {
    pub console_log_implemented: bool,
    pub console_buffer: Seq<u8>,
}

pub const FFA_ERROR: UInt32 = 0x84000060u32;
pub const FFA_SUCCESS: UInt32 = 0x84000061u32;

pub const NOT_SUPPORTED: i32 = -1i32;
pub const INVALID_PARAMETERS: i32 = -2i32;
pub const RETRY: i32 = -7i32;

pub open spec fn IsFfaConsoleLogImplemented(s: S) -> bool;

pub open spec fn ConsoleCharactersLogged(old_s: S, new_s: S, character_regs: Seq<UInt64>, count: int) -> bool;

} // verus!
