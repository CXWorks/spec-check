pub open spec fn sbi_pmu_counter_stop_spec(counter_idx_base: unsigned long, counter_idx_mask: unsigned long, stop_flags: unsigned long, result: Result<(), SbiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AllCountersValid(old_s, counter_idx_base, counter_idx_mask) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (stop_flags[63..2] != 0 ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (AnyCounterStopped(old_s, counter_idx_base, counter_idx_mask) ==> ResultEqual(result, SBI_ERR_ALREADY_STOPPED))
  && (stop_flags[1] == 1 && !SnapshotShmemAvailable(old_s) ==> ResultEqual(result, SBI_ERR_NO_SHMEM))
  && (result == SBI_SUCCESS ==> ResultEqual(result, SBI_SUCCESS))
  && (result == SBI_SUCCESS ==> forall idx in CounterSet(old_s, counter_idx_base, counter_idx_mask): CounterIsStopped(new_s, idx))
  && (result == SBI_SUCCESS && stop_flags[0] == 1 ==> forall idx in CounterSet(old_s, counter_idx_base, counter_idx_mask): CounterEventMappingIsReset(new_s, idx))
  && (result == SBI_SUCCESS && stop_flags[1] == 1 ==> forall idx in CounterSet(old_s, counter_idx_base, counter_idx_mask): SnapshotShmem(new_s).counter_value[idx] == CounterValue(new_s, idx))
  && (result == SBI_SUCCESS && stop_flags[1] == 1 ==> forall idx not in CounterSet(old_s, counter_idx_base, counter_idx_mask): SnapshotShmem(new_s).counter_value[idx] == SnapshotShmem(old_s).counter_value[idx])
  && (result == SBI_SUCCESS && stop_flags[1] == 1 ==> SnapshotShmem(new_s).overflow_bitmap == CounterOverflowBitmap(new_s))
  && ((AllCountersValid(old_s, counter_idx_base, counter_idx_mask) &&
       !(stop_flags[63..2] != 0) &&
       !(AnyCounterStopped(old_s, counter_idx_base, counter_idx_mask)) &&
       !(stop_flags[1] == 1 && !SnapshotShmemAvailable(old_s)))
    ==> result == SBI_SUCCESS)
  && (result != SBI_SUCCESS
    ==> forall idx in CounterSet(old_s, counter_idx_base, counter_idx_mask): CounterIsStopped(new_s, idx) == CounterIsStopped(old_s, idx))
  && (result != SBI_SUCCESS
    ==> forall idx in CounterSet(old_s, counter_idx_base, counter_idx_mask): CounterEventMappingIsReset(new_s, idx) == CounterEventMappingIsReset(old_s, idx))
  && (result != SBI_SUCCESS
    ==> forall idx in CounterSet(old_s, counter_idx_base, counter_idx_mask): SnapshotShmem(new_s).counter_value[idx] == SnapshotShmem(old_s).counter_value[idx])
  && (result != SBI_SUCCESS
    ==> forall idx not in CounterSet(old_s, counter_idx_base, counter_idx_mask): SnapshotShmem(new_s).counter_value[idx] == SnapshotShmem(old_s).counter_value[idx])
  && (result != SBI_SUCCESS
    ==> SnapshotShmem(new_s).overflow_bitmap == SnapshotShmem(old_s).overflow_bitmap)
}