pub open spec fn sbi_pmu_counter_stop_spec(result: long, old_s: S, new_s: S) -> bool {
    (!AllCountersValid(old_s, counter_idx_base, counter_idx_mask) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (stop_flags[XLEN-1:2] != 0 ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (AnyCounterStopped(old_s, counter_idx_base, counter_idx_mask) ==> ResultEqual(result, SBI_ERR_ALREADY_STOPPED))
    && (stop_flags[1] == 1 && !SnapshotShmemAvailable() ==> ResultEqual(result, SBI_ERR_NO_SHMEM))
    && (ResultEqual(result, SBI_SUCCESS) ==> (forall idx in CounterSet(counter_idx_base, counter_idx_mask): CounterIsStopped(idx)))
    && (stop_flags[0] == 1 ==> (forall idx in CounterSet(counter_idx_base, counter_idx_mask): CounterEventMappingIsReset(idx)))
    && (stop_flags[1] == 1 ==> (forall idx in CounterSet(counter_idx_base, counter_idx_mask): SnapshotShmem.counter_value[idx] == CounterValue(idx)))
    && (stop_flags[1] == 1 ==> (forall idx not in CounterSet(counter_idx_base, counter_idx_mask): SnapshotShmem.counter_value[idx] == old_s.SnapshotShmem.counter_value[idx]))
    && (stop_flags[1] == 1 ==> SnapshotShmem.overflow_bitmap == CounterOverflowBitmap())
}