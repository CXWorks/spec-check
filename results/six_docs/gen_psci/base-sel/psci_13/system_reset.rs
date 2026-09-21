pub open spec fn system_reset_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == PSCI_SUCCESS ==> (old_s.cpu_state == new_s.cpu_state))
    && (result == PSCI_NOT_SUPPORTED ==> (old_s.cpu_state == new_s.cpu_state))
    && (result == PSCI_INVALID_PARAMETERS ==> (old_s.cpu_state == new_s.cpu_state))
    && (result == PSCI_DENIED ==> (old_s.cpu_state == new_s.cpu_state))
    && (result == PSCI_ALREADY_ON ==> (old_s.cpu_state == new_s.cpu_state))
    && (result == PSCI_ON_PENDING ==> (old_s.cpu_state == new_s.cpu_state))
    && (result == PSCI_INTERNAL_FAILURE ==> (old_s.cpu_state == new_s.cpu_state))
    && (result == PSCI_NOT_PRESENT ==> (old_s.cpu_state == new_s.cpu_state))
    && (result == PSCI_DISABLED ==> (old_s.cpu_state == new_s.cpu_state))
    && (result == PSCI_INVALID_ADDRESS ==> (old_s.cpu_state == new_s.cpu_state))
}