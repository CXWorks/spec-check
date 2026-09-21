pub open spec fn system_off2_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == PSCI_INVALID_PARAMETERS ==> (true))
    && (result != PSCI_INVALID_PARAMETERS ==> (result == PSCI_DENIED || result == PSCI_NOT_SUPPORTED || result == PSCI_INTERNAL_FAILURE || result == PSCI_ALREADY_ON || result == PSCI_ON_PENDING || result == PSCI_DISABLED || result == PSCI_NOT_PRESENT || result == PSCI_INVALID_ADDRESS))
}