use vstd::prelude::*;
verus! {

pub type PsciReturnCode = i32;

pub type Core = u64;

pub type AffinityStateValue = u8;

pub struct S {
    pub system_suspend_implemented: bool,
    pub entry_point: u64,
    pub context: u64,
}

pub const PSCI_SUCCESS: PsciReturnCode = 0;
pub const NOT_SUPPORTED: PsciReturnCode = -1;
pub const DENIED: PsciReturnCode = -3;
pub const INVALID_ADDRESS: PsciReturnCode = -9;

pub const OFF: AffinityStateValue = 1;

pub const entry_point_address: u64 = 0x8000_0000;
pub const context_id: u64 = 0x1234;

pub open spec fn SystemSuspendImplemented(s: S) -> bool;

pub open spec fn ResultEqual(a: PsciReturnCode, b: PsciReturnCode) -> bool;

pub open spec fn EntryPointKnownInvalid(s: S, addr: u64) -> bool;

pub open spec fn CallingCore() -> Core;

pub open spec fn AffinityState(c: Core) -> AffinityStateValue;

pub open spec fn SystemInDeepestPowerdownState(s: S) -> bool;

pub open spec fn CoreResumesAtEntryPoint(c: Core, addr: u64, ctx: u64, s: S) -> bool;

} // verus!
