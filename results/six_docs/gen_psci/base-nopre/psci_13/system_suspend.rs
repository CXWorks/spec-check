pub open spec fn system_suspend_spec(result: PsciReturnCode, old_s: S, new_s: S) -> bool {
    (result == PSCI_NOT_SUPPORTED ==> (true))
    && (result == PSCI_INVALID_ADDRESS ==> (true))
    && (result == PSCI_DENIED ==> (true))
    && (result == PSCI_SUCCESS ==> (true))
}