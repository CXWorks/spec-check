use vstd::prelude::*;
verus! {

pub type Address = u64;
pub type UInt64 = u64;
pub type PsciReturnCode = i32;
pub type Core = u64;
pub type AffinityStateValue = u32;

pub struct S {
    pub dummy: u64,
}

pub const PSCI_SUCCESS: PsciReturnCode = 0;
pub const NOT_SUPPORTED: PsciReturnCode = -1;
pub const INVALID_PARAMETERS: PsciReturnCode = -2;
pub const DENIED: PsciReturnCode = -3;
pub const INVALID_ADDRESS: PsciReturnCode = -9;

pub const ON: AffinityStateValue = 0;
pub const OFF: AffinityStateValue = 1;
pub const ON_PENDING: AffinityStateValue = 2;

pub open spec fn SystemSuspendImplemented(s: S) -> bool;

pub open spec fn ResultEqual(a: PsciReturnCode, b: PsciReturnCode) -> bool;

pub open spec fn EntryPointKnownInvalid(s: S, entry_point_address: Address) -> bool;

pub open spec fn CallingCore(s: S) -> Core;

pub open spec fn AffinityState(c: Core) -> AffinityStateValue;

pub open spec fn SystemInDeepestPowerdownState(s: S) -> bool;

pub open spec fn CoreResumesAtEntryPoint(s: S, c: Core, entry_point_address: Address, context_id: UInt64) -> bool;

} // verus!
