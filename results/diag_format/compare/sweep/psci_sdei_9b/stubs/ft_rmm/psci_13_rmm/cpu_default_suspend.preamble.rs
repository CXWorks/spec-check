use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type Address = u64;

pub type PsciReturnCode = i32;

pub type Platform = u64;

pub struct S {
    pub pc: u64,
}

pub const SUCCESS: PsciReturnCode = 0;

pub const INVALID_ADDRESS: PsciReturnCode = -9;

#[allow(non_upper_case_globals)]
pub const platform: Platform = 0;

pub open spec fn IsKnownUnavailableToCaller(s: S, addr: Address) -> bool;

pub open spec fn ResultEqual(a: PsciReturnCode, b: PsciReturnCode) -> bool;

pub open spec fn ReturnedAtNextInstruction(s: S) -> bool;

pub open spec fn ResumedAt(s: S, addr: Address) -> bool;

pub open spec fn ContextIdPresented(s: S, context_id: UInt64) -> bool;

pub open spec fn AllCoresInDefaultSuspend(s: S) -> bool;

pub open spec fn ThermallyCritical(s: S, p: Platform) -> bool;

} // verus!
