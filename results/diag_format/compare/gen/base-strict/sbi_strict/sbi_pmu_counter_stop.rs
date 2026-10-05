pub open spec fn sbi_pmu_counter_stop_spec(result: SbiErrorCode, old_s: S, new_s: S, counter_idx_base: UInt, counter_idx_mask: UInt, stop_flags: PmuCounterStopFlags) -> bool {
    (exists|i: UInt| CounterInSet(counter_idx_base, counter_idx_mask, i) && !IsValidCounter(i) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (Bits(stop_flags, XLEN - 1, 2) != 0 ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (exists|i: UInt| CounterInSet(counter_idx_base, counter_idx_mask, i) && CounterIsStopped(i) ==> ResultEqual(result, SBI_ERR_ALREADY_STOPPED))
    && (Bits(stop_flags, 1, 1) == 1 && !SnapshotShmemAvailable() ==> ResultEqual(result, SBI_ERR_NO_SHMEM))
    && (ResultEqual(result, SBI_SUCCESS) ==> (forall|i: UInt| CounterInSet(counter_idx_base, counter_idx_mask, i) ==> CounterIsStopped(i)))
    && (Bits(stop_flags, 0, 0) == 1 ==> (forall|i: UInt| CounterInSet(counter_idx_base, counter_idx_mask, i) ==> CounterEventMappingIsReset(i)))
    && (Bits(stop_flags, 1, 1) == 1 ==> (forall|i: UInt| CounterInSet(counter_idx_base, counter_idx_mask, i) ==> SnapshotShmemCounterValue(i) == CounterValue(i)))
    && (Bits(stop_flags, 1, 1) == 1 ==> (forall|i: UInt| !CounterInSet(counter_idx_base, counter_idx_mask, i) ==> SnapshotShmemCounterValueUnchanged(i)))
    && (Bits(stop_flags, 1, 1) == 1 ==> SnapshotShmemOverflowBitmapUpdated())
}