pub open spec fn system_suspend_spec(result: int, old_s: S, new_s: S) -> bool {
    (!CpuIsValid(old_s, 0u64) ==> result == PSCI_INVALID_ADDRESS)
    && (CpuIsValid(old_s, 0u64) && !CpuIsOn(old_s, 0u64) ==> result == PSCI_DENIED)
    && (CpuIsValid(old_s, 0u64) && CpuIsOn(old_s, 0u64) ==> result == PSCI_NOT_SUPPORTED)
    && (CpuIsValid(old_s, 0u64) && CpuIsOn(old_s, 0u64) && result == PSCI_SUCCESS ==> CpuIsOn(new_s, 0u64))
}