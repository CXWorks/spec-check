pub open spec fn cpu_suspend_spec(power_state: PowerState, entry_point_address: Address, context_id: UInt64, result: PsciReturnCode, old_s: S, new_s: S) -> bool {
  (!IsValidPowerState(old_s, power_state) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (IsOsInitiatedMode(old_s) && RequestsHigherThanCoreLevel(old_s, power_state) && AnyChildInIncompatibleLowPowerState(old_s, power_state) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (IsOsInitiatedMode(old_s) && RequestsHigherThanCoreLevel(old_s, power_state) && AllIncompatibleCoresRunning(old_s, power_state) ==> ResultEqual(result, DENIED))
  && (IsKnownUnavailableAddress(old_s, entry_point_address) ==> ResultEqual(result, INVALID_ADDRESS))
  && (result == SUCCESS && StateType(power_state) == 0 ==> ResultEqual(result, SUCCESS))
  && (result == SUCCESS && StateType(power_state) == 0 ==> CoreStateUnchangedExceptTimersAndCpuInterface(new_s))
  && (result == SUCCESS && PowerdownDowngradedToStandby(old_s, power_state) ==> ResultEqual(result, SUCCESS))
  && (result == SUCCESS && PowerdownReturnedEarlyOnWakeupEvent(old_s, power_state) ==> ResultEqual(result, SUCCESS) || ResumedAt(new_s, entry_point_address))
  && (result == SUCCESS && StateType(power_state) == 1 && !ReturnedToCaller(old_s) ==> ResumedAt(new_s, entry_point_address) && ContextIdRegister(new_s) == context_id)
  && (result == SUCCESS && StateType(power_state) == 1 ==> CachesCleanedForPoweredDownNodes(new_s, power_state))
  && ((IsValidPowerState(old_s, power_state) &&
       !(IsOsInitiatedMode(old_s) && RequestsHigherThanCoreLevel(old_s, power_state) && AnyChildInIncompatibleLowPowerState(old_s, power_state)) &&
       !(IsOsInitiatedMode(old_s) && RequestsHigherThanCoreLevel(old_s, power_state) && AllIncompatibleCoresRunning(old_s, power_state)) &&
       !(IsKnownUnavailableAddress(old_s, entry_point_address)))
    ==> result == SUCCESS)
}