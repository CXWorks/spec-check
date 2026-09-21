pub open spec fn mem_protect_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == PSCI_NOT_SUPPORTED ==> !CpuIsValid(old_s, 0))
    && (result == PSCI_INVALID_PARAMETERS ==> (result != 0 && result != 1))
    && (result == 0 ==> (old_s.cpu_state[0] == 0))
    && (result == 1 ==> (old_s.cpu_state[0] == 1))
    && (result == 0 ==> new_s.cpu_state[0] == 0)
    && (result == 1 ==> new_s.cpu_state[0] == 1)
    && (result == 0 ==> new_s.cpu_state[1..64] == old_s.cpu_state[1..64])
    && (result == 1 ==> new_s.cpu_state[1..64] == old_s.cpu_state[1..64])
}