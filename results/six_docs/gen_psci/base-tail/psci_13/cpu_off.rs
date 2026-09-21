pub open spec fn cpu_off_spec(result: int, old_s: S, new_s: S) -> bool {
    (!CpuIsValid(old_s, old_s.cpu_state[0 as usize]) ==> result == PSCI_INVALID_PARAMETERS)
    && (!CpuIsOn(old_s, old_s.cpu_state[0 as usize]) ==> result == PSCI_DENIED)
    && (CpuIsValid(old_s, old_s.cpu_state[0 as usize]) && CpuIsOn(old_s, old_s.cpu_state[0 as usize]) ==> result == PSCI_SUCCESS)
}