use vstd::prelude::*;
verus! {

pub type Address = u64;
pub type UInt64 = u64;
pub type PsciReturnCode = i32;
pub type CorePowerState = u8;
pub type PlatformPowerState = u64;

pub const NOT_SUPPORTED: PsciReturnCode = -1;
pub const INVALID_ADDRESS: PsciReturnCode = -9;
pub const DENIED: PsciReturnCode = -3;

pub const OFF: CorePowerState = 1;

pub struct S {
    pub dummy: int,
}

pub open spec fn SystemSuspendImplemented(s: S) -> bool;
pub open spec fn ResultEqual(a: PsciReturnCode, b: PsciReturnCode) -> bool;
pub open spec fn IsKnownUnavailableToCaller(s: S, addr: Address) -> bool;
pub open spec fn CallingCore(s: S) -> int;
pub open spec fn CoreState(s: S, core: int) -> CorePowerState;
pub open spec fn SystemPowerState(s: S) -> PlatformPowerState;
pub open spec fn DeepestPlatformPowerdownState(s: S) -> PlatformPowerState;

} // verus!
