pub open spec fn affinity_info_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (!IsAffinityInstancePresent(old_s, lowest_affinity_level, target_affinity) ==> ResultEqual(result, RSI_ERROR_INPUT))
    && (PsciVersion() >= 1.0 && lowest_affinity_level > 0 && !SupportsAffinityLevelAboveZero() ==> ResultEqual(result, RSI_ERROR_INPUT))
    && (IsAffinityInstanceDisabled(old_s, lowest_affinity_level, target_affinity) ==> ResultEqual(result, RSI_ERROR_INPUT))
    && (Exists core in AffinityInstance(old_s, lowest_affinity_level, target_affinity) such that (CoreEnabledByCpuOn(core) || IsColdBootPrimaryCore(core)) && !CoreCalledCpuOff(core) ==> ResultEqual(result, RSI_SUCCESS))
    && (For all cores in AffinityInstance(old_s, lowest_affinity_level, target_affinity), CoreCalledCpuOff(core) && CpuOffProcessed(core) ==> ResultEqual(result, RSI_SUCCESS))
    && (Exists core in AffinityInstance(old_s, lowest_affinity_level, target_affinity) such that CoreState(core) == ON_PENDING, and all other cores in the instance are OFF ==> ResultEqual(result, RSI_SUCCESS))
}