pub open spec fn psci_stat_count_spec(result: u64, old_s: S, new_s: S) -> bool {
    (result == 0)
}