pub open spec fn cpu_suspend_spec(result: PsciReturnCode, power_state: UInt32, entry_point_address: UInt64, context_id: UInt64, old_s: S, new_s: S) -> bool {
    (!PsciExtendedStateIdFormat(old_s) && (power_state & 0xFCFE_0000u32) != 0
        ==> result == PSCI_INVALID_PARAMETERS)
    && (PsciExtendedStateIdFormat(old_s) && (power_state & 0xB000_0000u32) != 0
        ==> result == PSCI_INVALID_PARAMETERS)
    && (!PowerStateIsValid(old_s, power_state)
        ==> result == PSCI_INVALID_PARAMETERS)
    && (PsciOsInitiatedMode(old_s)
        && PowerStateTargetsHigherThanCoreLevel(old_s, power_state)
        && ChildInIncompatibleLowPowerState(old_s, power_state)
        ==> result == PSCI_INVALID_PARAMETERS)
    && (PowerStateIsValid(old_s, power_state)
        && PowerStateIsPowerdown(old_s, power_state)
        && EntryPointAddressKnownInvalid(old_s, entry_point_address)
        ==> result == PSCI_INVALID_ADDRESS)
    && (PsciOsInitiatedMode(old_s)
        && PowerStateTargetsHigherThanCoreLevel(old_s, power_state)
        && IncompatibleCoresAllRunning(old_s, power_state)
        ==> result == PSCI_DENIED)
    && ((PowerStateIsValid(old_s, power_state)
        && !(PowerStateIsPowerdown(old_s, power_state)
            && EntryPointAddressKnownInvalid(old_s, entry_point_address))
        && !(PsciOsInitiatedMode(old_s)
            && PowerStateTargetsHigherThanCoreLevel(old_s, power_state)
            && (ChildInIncompatibleLowPowerState(old_s, power_state)
                || IncompatibleCoresAllRunning(old_s, power_state))))
        ==> (
            (!PowerStateIsPowerdown(old_s, power_state)
                ==> result == PSCI_SUCCESS
                    && CoreStateUnchangedOnStandbyReturn(old_s, new_s))
            && (PowerStateIsPowerdown(old_s, power_state)
                ==> (result == PSCI_SUCCESS
                        && CoreStateUnchangedOnStandbyReturn(old_s, new_s))
                    || (ResumedAtEntryPoint(new_s, entry_point_address)
                        && WakeupContextId(new_s) == context_id))
        ))
}
