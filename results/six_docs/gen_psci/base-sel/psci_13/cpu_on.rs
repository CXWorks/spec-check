pub open spec fn cpu_on_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == PSCI_INVALID_PARAMETERS ==> (old_s.cpu_state[0 as usize] != 0))
    && (result == PSCI_INVALID_ADDRESS ==> (old_s.cpu_state[0 as usize] != 0))
    && (result == PSCI_ALREADY_ON ==> (old_s.cpu_state[0 as usize] != 0))
    && (result == PSCI_ON_PENDING ==> (old_s.cpu_state[0 as usize] != 0))
    && (result == PSCI_INTERNAL_FAILURE ==> (old_s.cpu_state[0 as usize] != 0))
    && (result == PSCI_DENIED ==> (old_s.cpu_state[0 as usize] != 0))
    && (result == PSCI_SUCCESS ==> (new_s.cpu_state[0 as usize] == 0))
}