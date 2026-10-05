pub open spec fn psci_stat_residency_spec(result: UInt, old_s: S, new_s: S) -> bool {
    (!StatFunctionsImplemented(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsPresentNode(old_s, target_cpu(old_s)) ==> ResultEqual(result, 0))
    && (!NodeSupportsState(old_s, StatNode(old_s, power_state(old_s)), StatLocalState(power_state(old_s))) ==> ResultEqual(result, 0))
    && (IsStatResidency(fid(old_s)) ==> ResultEqual(result, StateResidencyUs(old_s, StatNode(old_s, power_state(old_s)), StatLocalState(power_state(old_s))) mod 2u64.pow(ResultBits(fid(old_s)) as int)))
}