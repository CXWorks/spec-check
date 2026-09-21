pub open spec fn cpu_freeze_spec(result: int, old_s: S, new_s: S) -> bool {
    (!CpuIsValid(old_s, 0u64) ==> result == PSCI_NOT_SUPPORTED)
    && (CpuIsOn(old_s, 0u64) ==> result == PSCI_DENIED)
    && (result == PSCI_SUCCESS ==> CpuIsOn(old_s, 0u64) && CpuIsOn(new_s, 0u64))
}