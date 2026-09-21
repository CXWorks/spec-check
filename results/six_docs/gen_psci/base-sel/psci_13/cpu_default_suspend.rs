pub open spec fn cpu_default_suspend_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == PSCI_INVALID_ADDRESS ==> (new_s.cpu_state == old_s.cpu_state))
    && (result == PSCI_SUCCESS ==> (new_s.cpu_state == old_s.cpu_state))
    && (result == PSCI_NOT_SUPPORTED ==> (new_s.cpu_state == old_s.cpu_state))
    && (result == PSCI_DENIED ==> (new_s.cpu_state == old_s.cpu_state))
    && (result == PSCI_ALREADY_ON ==> (new_s.cpu_state == old_s.cpu_state))
    && (result == PSCI_ON_PENDING ==> (new_s.cpu_state == old_s.cpu_state))
    && (result == PSCI_INTERNAL_FAILURE ==> (new_s.cpu_state == old_s.cpu_state))
    && (result == PSCI_NOT_PRESENT ==> (new_s.cpu_state == old_s.cpu_state))
    && (result == PSCI_DISABLED ==> (new_s.cpu_state == old_s.cpu_state))
    && (result == PSCI_INVALID_ADDRESS ==> (new_s.cpu_state == old_s.cpu_state))
}