pub open spec fn migrate_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == PSCI_NOT_SUPPORTED ==> true)
    && (result == PSCI_INVALID_PARAMETERS ==> true)
    && (result == PSCI_DENIED ==> true)
    && (result == PSCI_INTERNAL_FAILURE ==> true)
    && (result == PSCI_NOT_PRESENT ==> true)
    && (result == PSCI_SUCCESS ==> (old_s.cpu_state[0] == PSCI_HW_STATE_ON && new_s.cpu_state[0] == PSCI_HW_STATE_OFF))
}