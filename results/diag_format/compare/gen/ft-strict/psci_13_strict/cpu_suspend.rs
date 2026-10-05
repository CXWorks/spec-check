pub open spec fn cpu_suspend_spec(power_state: Bits32, entry_point_address: Address, context_id: UInt64, result: PsciReturnCode, old_s: S, new_s: S) -> bool {
  (!IsValidPowerState(old_s, power_state) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (IsOsInitiatedMode(old_s) && PowerLevel(old_s, power_state) > CORE_POWER_LEVEL && (exists|c: Core| IsChildOfNode(c, RequestedNode(old_s, power_state)) && IsInLocalLowPowerState(c) && IsIncompatibleWithRequest(c, power_state)) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (StateType(old_s, power_state) == POWERDOWN && IsKnownUnavailableToCaller(old_s, entry_point_address) ==> ResultEqual(result, INVALID_ADDRESS))
  && (IsOsInitiatedMode(old_s) && PowerLevel(old_s, power_state) > CORE_POWER_LEVEL && (exists|c: Core| IsChildOfNode(c, RequestedNode(old_s, power_state)) && IsIncompatibleWithRequest(c, power_state)) && (forall|c: Core| (IsChildOfNode(c, RequestedNode(old_s, power_state)) && IsIncompatibleWithRequest(c, power_state)) ==> IsRunning(c)) ==> ResultEqual(result, DENIED))
  && (result == SUCCESS && StateType(old_s, power_state) == STANDBY ==> ResultEqual(result, SUCCESS))
  && (result == SUCCESS && StateType(old_s, power_state) == STANDBY ==> CoreStateUnchangedExceptTimersCpuInterfaceAndSmcRegisters(new_s, CurrentCore()))
  && (result == SUCCESS && StateType(old_s, power_state) == POWERDOWN ==> (ResultEqual(result, SUCCESS) || CoreRestartsAtEntryPoint(new_s, CurrentCore(), entry_point_address)))
  && ((result == SUCCESS && StateType(old_s, power_state) == POWERDOWN && CoreRestartsAtEntryPoint(new_s, CurrentCore(), entry_point_address)) ==> FirstNonSecureElReg0Equals(new_s, CurrentCore(), context_id))
  && (result == SUCCESS && IsPlatformCoordinatedMode(old_s) ==> EnteredStateNoDeeperThan(new_s, CurrentCore(), power_state))
  && (result == SUCCESS && IsOsInitiatedMode(old_s) ==> EnteredStateEquals(new_s, RequestedNode(old_s, power_state), power_state))
  && (result == SUCCESS && StateType(old_s, power_state) == POWERDOWN ==> CachesCleanedAndCoherencyManaged(new_s, RequestedNode(old_s, power_state)))
  && ((!(IsValidPowerState(old_s, power_state)) &&
       !(IsOsInitiatedMode(old_s) && PowerLevel(old_s, power_state) > CORE_POWER_LEVEL && (exists|c: Core| IsChildOfNode(c, RequestedNode(old_s, power_state)) && IsInLocalLowPowerState(c) && IsIncompatibleWithRequest(c, power_state))) &&
       !(StateType(old_s, power_state) == POWERDOWN && IsKnownUnavailableToCaller(old_s, entry_point_address)) &&
       !(IsOsInitiatedMode(old_s) && PowerLevel(old_s, power_state) > CORE_POWER_LEVEL && (exists|c: Core| IsChildOfNode(c, RequestedNode(old_s, power_state)) && IsIncompatibleWithRequest(c, power_state)) && (forall|c: Core| (IsChildOfNode(c, RequestedNode(old_s, power_state)) && IsIncompatibleWithRequest(c, power_state)) ==> IsRunning(c))))
    ==> result == SUCCESS)
}