use vstd::prelude::*;

verus! {

pub type SbiError = i64;

pub const SBI_SUCCESS: SbiError = 0;
pub const SBI_ERR_FAILED: SbiError = -1;
pub const SBI_ERR_DENIED: SbiError = -4;

pub struct DebugConsoleState {
    pub output: Seq<u8>,
}

pub struct S {
    pub DebugConsole: DebugConsoleState,
}

pub open spec fn DebugConsoleWriteAllowed(s: S) -> bool;

pub open spec fn ResultEqual(a: SbiError, b: SbiError) -> bool;

pub open spec fn DebugConsoleIoError(byte: u8) -> bool;

pub open spec fn DebugConsoleByteWritten(byte: u8) -> bool;

pub open spec fn DebugConsoleGlobal() -> DebugConsoleState;

pub spec const DebugConsole: DebugConsoleState = DebugConsoleGlobal();

} // verus!
