pub open spec fn system_suspend_spec(result: PsciReturnCode, old_s: PsciState, new_s: PsciState) -> bool {
    (result == PSCI_NOT_SUPPORTED ==> true)
    && (result == PSCI_INVALID_ADDRESS ==> true)
    && (result == PSCI_DENIED ==> true)
    && (result == PSCI_SUCCESS ==> (new_s == old_s))
}