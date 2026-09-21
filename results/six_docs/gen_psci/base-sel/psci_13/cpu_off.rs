pub open spec fn cpu_off_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == PSCI_DENIED ==> old_s.cpu_state[0] as int == PSCI_HW_STATE_ON)
    && (result != PSCI_DENIED ==> result == PSCI_SUCCESS)
    && (result == PSCI_SUCCESS ==> new_s.cpu_state[0] as int == PSCI_HW_STATE_OFF)
}