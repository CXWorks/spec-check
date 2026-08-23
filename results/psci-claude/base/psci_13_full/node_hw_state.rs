pub open spec fn node_hw_state_spec(result: Int32, target_cpu: Bits64, power_level: UInt32, old_s: S, new_s: S) -> bool {
    (!CpuIsValid(old_s, target_cpu) ==> result as int == PSCI_INVALID_PARAMETERS)
    && (CpuIsValid(old_s, target_cpu) ==> (result as int == PSCI_HW_STATE_ON
        || result as int == PSCI_HW_STATE_OFF
        || result as int == PSCI_HW_STATE_STANDBY
        || result as int == PSCI_NOT_SUPPORTED))
    && (new_s == old_s)
}