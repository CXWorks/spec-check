pub open spec fn sbi_pmu_counter_stop_spec(counter_idx_base: UInt64, counter_idx_mask: UInt64, stop_flags: UInt64, result: SbiErrorCode, old_s: S, new_s: S) -> bool {
    (
        (CounterSetHasInvalidCounter(old_s, counter_idx_base, counter_idx_mask) || (stop_flags >> 2u64) != 0)
        ==> result != SBI_SUCCESS
    )
    && (
        (result == SBI_ERR_INVALID_PARAM)
        ==> (CounterSetHasInvalidCounter(old_s, counter_idx_base, counter_idx_mask) || (stop_flags >> 2u64) != 0)
    )
    && (
        CounterSetHasStoppedCounter(old_s, counter_idx_base, counter_idx_mask)
        ==> result != SBI_SUCCESS
    )
    && (
        (result == SBI_ERR_ALREADY_STOPPED)
        ==> CounterSetHasStoppedCounter(old_s, counter_idx_base, counter_idx_mask)
    )
    && (
        ((stop_flags & 2u64) != 0 && !SnapshotShmemAvailable(old_s))
        ==> result != SBI_SUCCESS
    )
    && (
        (result == SBI_ERR_NO_SHMEM)
        ==> ((stop_flags & 2u64) != 0 && !SnapshotShmemAvailable(old_s))
    )
    && (
        (result != SBI_SUCCESS) ==> new_s == old_s
    )
    && (
        (
            !CounterSetHasInvalidCounter(old_s, counter_idx_base, counter_idx_mask)
            && (stop_flags >> 2u64) == 0
            && !CounterSetHasStoppedCounter(old_s, counter_idx_base, counter_idx_mask)
            && !((stop_flags & 2u64) != 0 && !SnapshotShmemAvailable(old_s))
        )
        ==> (
            result == SBI_SUCCESS
            && (forall |i: int| 0 <= i < 64 && ((counter_idx_mask >> (i as u64)) & 1u64) == 1u64
                ==> CounterIsStopped(new_s, counter_idx_base as int + i))
            && ((stop_flags & 1u64) != 0 ==>
                (forall |i: int| 0 <= i < 64 && ((counter_idx_mask >> (i as u64)) & 1u64) == 1u64
                    ==> CounterEventMappingIsReset(new_s, counter_idx_base as int + i)))
            && ((stop_flags & 2u64) != 0 ==>
                (
                    (forall |i: int| 0 <= i < 64 && ((counter_idx_mask >> (i as u64)) & 1u64) == 1u64
                        ==> SnapshotCounterValue(new_s, counter_idx_base as int + i) == CounterValue(new_s, counter_idx_base as int + i))
                    && (forall |j: int| !(counter_idx_base as int <= j < counter_idx_base as int + 64
                            && ((counter_idx_mask >> ((j - counter_idx_base as int) as u64)) & 1u64) == 1u64)
                        ==> SnapshotCounterValue(new_s, j) == SnapshotCounterValue(old_s, j))
                    && SnapshotOverflowBitmapUpdated(old_s, new_s)
                ))
        )
    )
}
