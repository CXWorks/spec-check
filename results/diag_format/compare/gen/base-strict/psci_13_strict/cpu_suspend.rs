pub open spec fn cpu_suspend_spec(result: PsciReturnCode, old_s: S, new_s: S) -> bool {
    (!IsValidPowerState(power_state(old_s)) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (IsOsInitiatedMode(old_s) && PowerLevel(power_state(old_s)) > CORE_POWER_LEVEL && (exists|c: Core| IsChildOfNode(c, RequestedNode(power_state(old_s))) && IsInLocalLowPowerState(c) && IsIncompatibleWithRequest(c, power_state(old_s))) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (StateType(power_state(old_s)) == POWERDOWN && IsKnownUnavailableToCaller(entry_point_address(old_s)) ==> ResultEqual(result, INVALID_ADDRESS))
    && (IsOsInitiatedMode(old_s) && PowerLevel(power_state(old_s)) > CORE_POWER_LEVEL && (exists|c: Core| IsChildOfNode(c, RequestedNode(power_state(old_s))) && IsIncompatibleWithRequest(c, power_state(old_s))) && (forall|c: Core| (IsChildOfNode(c, RequestedNode(power_state(old_s))) && IsIncompatibleWithRequest(c, power_state(old_s))) ==> IsRunning(c)) ==> ResultEqual(result, DENIED))
    && (StateType(power_state(old_s)) == STANDBY ==> ResultEqual(result, SUCCESS))
    && (StateType(power_state(old_s)) == STANDBY ==> CoreStateUnchangedExceptTimersCpuInterfaceAndSmcRegisters(CurrentCore(old_s)))
    && (StateType(power_state(old_s)) == POWERDOWN ==> (ResultEqual(result, SUCCESS) || CoreRestartsAtEntryPoint(CurrentCore(old_s), entry_point_address(old_s))))
    && ((StateType(power_state(old_s)) == POWERDOWN && CoreRestartsAtEntryPoint(CurrentCore(old_s), entry_point_address(old_s))) ==> FirstNonSecureElReg0Equals(CurrentCore(old_s), context_id(old_s)))
    && (IsPlatformCoordinatedMode(old_s) ==> EnteredStateNoDeeperThan(CurrentCore(old_s), power_state(old_s)))
    && (IsOsInitiatedMode(old_s) ==> EnteredStateEquals(RequestedNode(power_state(old_s)), power_state(old_s)))
    && (StateType(power_state(old_s)) == POWERDOWN ==> CachesCleanedAndCoherencyManaged(RequestedNode(power_state(old_s))))
}