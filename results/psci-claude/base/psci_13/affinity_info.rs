pub open spec fn affinity_info_spec(result: int, old_s: S, new_s: S, target_affinity: Bits64, lowest_affinity_level: UInt32) -> bool {
    (!CpuIsValid(old_s, target_affinity) ==> result == PSCI_INVALID_PARAMETERS)
    && (CpuIsValid(old_s, target_affinity) && CpuIsOn(old_s, target_affinity)
            ==> result == PSCI_AFFINITY_LEVEL_ON)
    && (CpuIsValid(old_s, target_affinity) && !CpuIsOn(old_s, target_affinity)
            && CpuIsOnPending(old_s, target_affinity)
            ==> result == PSCI_AFFINITY_LEVEL_ON_PENDING)
    && (CpuIsValid(old_s, target_affinity) && !CpuIsOn(old_s, target_affinity)
            && !CpuIsOnPending(old_s, target_affinity)
            ==> result == PSCI_AFFINITY_LEVEL_OFF)
    && (new_s == old_s)
}