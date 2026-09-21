pub open spec fn system_off_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == PSCI_SUCCESS)
    && (old_s.cpu_state == new_s.cpu_state)
}