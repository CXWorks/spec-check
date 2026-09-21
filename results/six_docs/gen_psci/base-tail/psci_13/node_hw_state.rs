pub open spec fn node_hw_state_spec(result: int, old_s: S, new_s: S) -> bool {
    (!CpuIsValid(old_s, old_s.cmd_input_target_cpu) ==> result == PSCI_INVALID_PARAMETERS)
    && (!old_s.cmd_input_power_level == 0 ==> result == PSCI_INVALID_PARAMETERS)
    && (result == PSCI_NOT_SUPPORTED ==> true)
    && (result == PSCI_INVALID_PARAMETERS ==> true)
    && (result == PSCI_SUCCESS ==> (old_s.cmd_input_target_cpu == new_s.cmd_input_target_cpu) && (old_s.cmd_input_power_level == new_s.cmd_input_power_level))
}