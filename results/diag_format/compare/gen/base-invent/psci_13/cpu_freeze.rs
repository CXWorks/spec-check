pub open spec fn cpu_freeze_spec(result: PsciReturnCode, old_s: PsciState, new_s: PsciState) -> bool {
    (result == PSCI_NOT_SUPPORTED ==> !PsciFeatureEnabled(old_s, PSCI_FEATURE_CPU_FREEZE))
    && (result == PSCI_DENIED ==> PsciDenied(old_s))
    && (result == PSCI_SUCCESS ==> PsciFeatureEnabled(old_s, PSCI_FEATURE_CPU_FREEZE))
    && (result == PSCI_SUCCESS ==> new_s == old_s)
}