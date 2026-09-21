pub open spec fn system_reset_spec(result: int, old_s: S, new_s: S) -> bool {
    (!CpuIsValid(old_s, 0) ==> result == PSCI_INVALID_PARAMETERS)
    && (CpuIsValid(old_s, 0) ==> result == PSCI_SUCCESS)
    && (result == PSCI_SUCCESS ==> new_s.cpu_state == old_s.cpu_state)
}