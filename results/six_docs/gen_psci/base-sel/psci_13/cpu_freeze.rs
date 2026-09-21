pub open spec fn cpu_freeze_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == PSCI_NOT_SUPPORTED ==> true)
    && (result == PSCI_DENIED ==> true)
    && (result == PSCI_SUCCESS ==> (old_s.cpu_state == new_s.cpu_state))
}