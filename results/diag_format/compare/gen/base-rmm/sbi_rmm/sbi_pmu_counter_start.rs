pub open spec fn sbi_pmu_counter_start_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    let fid = old_s.a6 as int;
    let counter_idx_base = old_s.a0 as int;
    let counter_idx_mask = old_s.a1 as int;
    let start_flags = old_s.a2 as int;
    let initial_value = old_s.a3 as int;

    // Failure: FID must be 3
    (fid != 3 ==> result != sbiret::SBI_SUCCESS)
    // Failure: Reserved bits must be zero
    ((start_flags & !0x3) != 0 ==> result != sbiret::SBI_SUCCESS)
    // Failure: Mutually exclusive flags
    ((start_flags & SBI_PMU_START_SET_INIT_VALUE) != 0 && (start_flags & SBI_PMU_START_FLAG_INIT_SNAPSHOT) != 0 ==> result != sbiret::SBI_SUCCESS)
    // Failure: Snapshot flag requires shared memory to be set (assumed via state check)
    ((start_flags & SBI_PMU_START_FLAG_INIT_SNAPSHOT) != 0 && !old_s.shmem_set ==> result != sbiret::SBI_SUCCESS)

    // Success: All counters in the set are started
    (forall idx in CounterSet(counter_idx_base, counter_idx_mask): CounterIsStarted(idx, new_s))
    // Success: If init value flag is set, all counters in the set have the initial value
    ((start_flags & SBI_PMU_START_SET_INIT_VALUE) != 0 ==> forall idx in CounterSet(counter_idx_base, counter_idx_mask): CounterValue(idx, new_s) == initial_value)
    // Success: If init snapshot flag is set, all valid counters in the set have the snapshot value
    ((start_flags & SBI_PMU_START_FLAG_INIT_SNAPSHOT) != 0 ==> forall idx in ValidCounters(CounterSet(counter_idx_base, counter_idx_mask)): CounterValue(idx, new_s) == SnapshotShmemCounterValue(idx))
    // Success: If neither flag is set, all counters in the set retain their old values
    ((start_flags & SBI_PMU_START_SET_INIT_VALUE) == 0 && (start_flags & SBI_PMU_START_FLAG_INIT_SNAPSHOT) == 0 ==> forall idx in CounterSet(counter_idx_base, counter_idx_mask): CounterValue(idx, new_s) == CounterValue(idx, old_s))
}