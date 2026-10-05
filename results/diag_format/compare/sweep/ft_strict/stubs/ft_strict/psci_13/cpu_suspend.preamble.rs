use vstd::prelude::*;
verus! {

pub type Bits32 = u32;
pub type Address = u64;
pub type UInt64 = u64;
pub type PsciReturnCode = i32;
pub type PowerStateType = u8;

pub struct Core {
    pub id: u64,
}

pub struct Node {
    pub id: u64,
}

pub struct S {
    pub dummy: u64,
}

pub const SUCCESS: PsciReturnCode = 0;
pub const INVALID_PARAMETERS: PsciReturnCode = -2;
pub const DENIED: PsciReturnCode = -3;
pub const INVALID_ADDRESS: PsciReturnCode = -9;

pub const STANDBY: PowerStateType = 0;
pub const POWERDOWN: PowerStateType = 1;

pub const CORE_POWER_LEVEL: u64 = 0;

pub open spec fn IsValidPowerState(s: S, power_state: Bits32) -> bool;
pub open spec fn ResultEqual(result: PsciReturnCode, code: PsciReturnCode) -> bool;
pub open spec fn IsOsInitiatedMode(s: S) -> bool;
pub open spec fn IsPlatformCoordinatedMode(s: S) -> bool;
pub open spec fn PowerLevel(s: S, power_state: Bits32) -> u64;
pub open spec fn IsChildOfNode(c: Core, n: Node) -> bool;
pub open spec fn RequestedNode(s: S, power_state: Bits32) -> Node;
pub open spec fn IsInLocalLowPowerState(c: Core) -> bool;
pub open spec fn IsIncompatibleWithRequest(c: Core, power_state: Bits32) -> bool;
pub open spec fn StateType(s: S, power_state: Bits32) -> PowerStateType;
pub open spec fn IsKnownUnavailableToCaller(s: S, entry_point_address: Address) -> bool;
pub open spec fn IsRunning(c: Core) -> bool;
pub open spec fn CoreStateUnchangedExceptTimersCpuInterfaceAndSmcRegisters(s: S, c: Core) -> bool;
pub open spec fn CurrentCore() -> Core;
pub open spec fn CoreRestartsAtEntryPoint(s: S, c: Core, entry_point_address: Address) -> bool;
pub open spec fn FirstNonSecureElReg0Equals(s: S, c: Core, context_id: UInt64) -> bool;
pub open spec fn EnteredStateNoDeeperThan(s: S, c: Core, power_state: Bits32) -> bool;
pub open spec fn EnteredStateEquals(s: S, n: Node, power_state: Bits32) -> bool;
pub open spec fn CachesCleanedAndCoherencyManaged(s: S, n: Node) -> bool;

} // verus!
