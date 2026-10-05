pub open spec fn psci_stat_residency_spec(target_cpu: u64, power_state: u32, result: u64, old_s: S, new_s: S) -> bool {
    (!PsciStatNodeStateIsValid(old_s, target_cpu, power_state) ==> result == 0)
    && (new_s == old_s)
}
