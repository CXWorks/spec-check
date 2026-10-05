pub open spec fn cpu_suspend_spec(result: PsciReturnCode, old_s: S, new_s: S) -> bool {
    (!IsValidPowerState(old_s, power_state) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (IsOsInitiatedMode(old_s) && RequestsHigherThanCoreLevel(old_s, power_state) && AnyChildInIncompatibleLowPowerState(old_s, power_state) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (IsOsInitiatedMode(old_s) && RequestsHigherThanCoreLevel(old_s, power_state) && AllIncompatibleCoresRunning(old_s, power_state) ==> ResultEqual(result, DENIED))
    && (IsKnownUnavailableAddress(old_s, entry_point_address) ==> ResultEqual(result, INVALID_ADDRESS))
    && (StateType(old_s, power_state) == 0 ==> ResultEqual(result, SUCCESS))
    && (StateType(old_s, power_state) == 0 ==> CoreStateUnchangedExceptTimersAndCpuInterface(old_s, new_s))
    && (PowerdownDowngradedToStandby(old_s, power_state) ==> ResultEqual(result, SUCCESS))
    && (PowerdownReturnedEarlyOnWakeupEvent(old_s, power_state) ==> ResultEqual(result, SUCCESS) || ResumedAt(old_s, entry_point_address))
    && (StateType(old_s, power_state) == 1 && !ReturnedToCaller(old_s) ==> ResumedAt(old_s, entry_point_address) && ContextIdRegister(old_s) == context_id)
    && (StateType(old_s, power_state) == 1 ==> CachesCleanedForPoweredDownNodes(old_s, new_s, power_state))
}