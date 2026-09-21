pub open spec fn system_reset2_spec(result: PsciCommandReturnCode, old_s: PsciState, new_s: PsciState) -> bool {
    (result == PSCI_NOT_SUPPORTED ==> (PsciFeatures(old_s).system_reset2 == PSCI_FEATURE_NOT_SUPPORTED))
    && (result != PSCI_NOT_SUPPORTED ==> (PsciFeatures(old_s).system_reset2 == PSCI_FEATURE_SUPPORTED))
    && (result == PSCI_ERROR_INVALID_PARAMETERS ==> (old_s.cmd_input_reset_type & 0x80000000 == 0 && (old_s.cmd_input_reset_type != 0)))
    && (result == PSCI_SUCCESS ==> (old_s.cmd_input_reset_type & 0x80000000 == 0 && old_s.cmd_input_reset_type == 0))
    && (result == PSCI_SUCCESS ==> (new_s.cpu_state == old_s.cpu_state))
    && (result == PSCI_SUCCESS ==> (new_s.interrupt_state == old_s.interrupt_state))
    && (result == PSCI_SUCCESS ==> (new_s.mmu_state == old_s.mmu_state))
    && (result == PSCI_SUCCESS ==> (new_s.smmu_state == old_s.smmu_state))
}