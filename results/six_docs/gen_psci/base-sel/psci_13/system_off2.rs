pub open spec fn system_off2_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == PSCI_INVALID_PARAMETERS ==> (true))
    && (result == PSCI_SUCCESS ==> (true))
    && (result == PSCI_NOT_SUPPORTED ==> (true))
    && (result == PSCI_DENIED ==> (true))
    && (result == PSCI_ALREADY_ON ==> (true))
    && (result == PSCI_ON_PENDING ==> (true))
    && (result == PSCI_INTERNAL_FAILURE ==> (true))
    && (result == PSCI_NOT_PRESENT ==> (true))
    && (result == PSCI_DISABLED ==> (true))
    && (result == PSCI_INVALID_ADDRESS ==> (true))
    && (true)
}