pub open spec fn psci_stat_count_spec(target_cpu: UInt64, power_state: UInt32, result: UInt64, old_s: S, new_s: S) -> bool {
    (!IsValidPsciStatNodeAndState(old_s, target_cpu, power_state) ==> result == 0)
    && (IsValidPsciStatNodeAndState(old_s, target_cpu, power_state) ==> (result == PsciStatCountOf(old_s, target_cpu, power_state) && new_s == old_s))
}
