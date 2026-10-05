use vstd::prelude::*;

verus! {

pub type PsciReturnCode = i32;
pub type PowerState = u32;
pub type PowerLevelValue = u32;
pub type StateTypeValue = u8;
pub type Address = u64;
pub type ContextId = u64;

pub struct S {
    pub power_state_field: PowerState,
    pub entry_point_address_field: Address,
    pub context_id_field: ContextId,
}

pub struct Core {
    pub id: nat,
}

pub struct Node {
    pub id: nat,
}

pub const SUCCESS: PsciReturnCode = 0;
pub const INVALID_PARAMETERS: PsciReturnCode = -2;
pub const DENIED: PsciReturnCode = -3;
pub const INVALID_ADDRESS: PsciReturnCode = -9;

pub const CORE_POWER_LEVEL: PowerLevelValue = 0;

pub const STANDBY: StateTypeValue = 0;
pub const POWERDOWN: StateTypeValue = 1;

pub open spec fn power_state(s: S) -> PowerState;
pub open spec fn entry_point_address(s: S) -> Address;
pub open spec fn context_id(s: S) -> ContextId;

pub open spec fn IsValidPowerState(ps: PowerState) -> bool;
pub open spec fn ResultEqual(a: PsciReturnCode, b: PsciReturnCode) -> bool;
pub open spec fn IsOsInitiatedMode(s: S) -> bool;
pub open spec fn IsPlatformCoordinatedMode(s: S) -> bool;
pub open spec fn PowerLevel(ps: PowerState) -> PowerLevelValue;
pub open spec fn RequestedNode(ps: PowerState) -> Node;
pub open spec fn IsChildOfNode(c: Core, n: Node) -> bool;
pub open spec fn IsInLocalLowPowerState(c: Core) -> bool;
pub open spec fn IsIncompatibleWithRequest(c: Core, ps: PowerState) -> bool;
pub open spec fn StateType(ps: PowerState) -> StateTypeValue;
pub open spec fn IsKnownUnavailableToCaller(addr: Address) -> bool;
pub open spec fn IsRunning(c: Core) -> bool;
pub open spec fn CurrentCore(s: S) -> Core;
pub open spec fn CoreStateUnchangedExceptTimersCpuInterfaceAndSmcRegisters(c: Core) -> bool;
pub open spec fn CoreRestartsAtEntryPoint(c: Core, addr: Address) -> bool;
pub open spec fn FirstNonSecureElReg0Equals(c: Core, ctx: ContextId) -> bool;
pub open spec fn EnteredStateNoDeeperThan(c: Core, ps: PowerState) -> bool;
pub open spec fn EnteredStateEquals(n: Node, ps: PowerState) -> bool;
pub open spec fn CachesCleanedAndCoherencyManaged(n: Node) -> bool;

} // verus!
