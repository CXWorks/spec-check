pub open spec fn mem_protect_check_range_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == PSCI_SUCCESS ==> (new_s.cpu_state == old_s.cpu_state))
    && (result == PSCI_DENIED ==> (new_s.cpu_state == old_s.cpu_state))
    && (result == PSCI_NOT_SUPPORTED ==> (new_s.cpu_state == old_s.cpu_state))
    && (result == PSCI_INVALID_ADDRESS ==> (new_s.cpu_state == old_s.cpu_state))
    && (result == PSCI_INTERNAL_FAILURE ==> (new_s.cpu_state == old_s.cpu_state))
    && (result == PSCI_INVALID_PARAMETERS ==> (new_s.cpu_state == old_s.cpu_state))
    && (result == PSCI_DENIED ==> (new_s.cpu_state == old_s.cpu_state))
}