pub open spec fn affinity_info_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == PSCI_INVALID_PARAMETERS ==> (old_s.cpu_state[0] == 0))
    && (result == PSCI_DISABLED ==> (old_s.cpu_state[0] == 0))
    && (result == PSCI_SUCCESS ==> (new_s.cpu_state == old_s.cpu_state))
}