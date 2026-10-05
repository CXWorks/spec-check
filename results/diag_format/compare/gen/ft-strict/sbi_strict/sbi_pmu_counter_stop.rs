pub open spec fn sbi_pmu_counter_stop_spec(counter_idx_base: UInt, counter_idx_mask: UInt, stop_flags: PmuCounterStopFlags, result: SbiErrorCode, old_s: S, new_s: S) -> bool {
  (exists i: UInt | CounterInSet(old_s, counter_idx_base, counter_idx_mask, i) && !IsValidCounter(old_s, i) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (Bits(stop_flags, XLEN - 1, 2) != 0 ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (exists i: UInt | CounterInSet(old_s, counter_idx_base, counter_idx_mask, i) && CounterIsStopped(old_s, i) ==> ResultEqual(result, SBI_ERR_ALREADY_STOPPED))
  && (Bits(stop_flags, 1, 1) == 1 && !SnapshotShmemAvailable(old_s) ==> ResultEqual(result, SBI_ERR_NO_SHMEM))
  && (result == SBI_SUCCESS ==> ResultEqual(result, SBI_SUCCESS))
  && (result == SBI_SUCCESS ==> forall i: UInt | CounterInSet(old_s, counter_idx_base, counter_idx_mask, i) ==> CounterIsStopped(new_s, i))
  && (result == SBI_SUCCESS && Bits(stop_flags, 0, 0) == 1 ==> (forall i: UInt | CounterInSet(old_s, counter_idx_base, counter_idx_mask, i) ==> CounterEventMappingIsReset(new_s, i)))
  && (result == SBI_SUCCESS && Bits(stop_flags, 1, 1) == 1 ==> (forall i: UInt | CounterInSet(old_s, counter_idx_base, counter_idx_mask, i) ==> SnapshotShmemCounterValue(new_s, i) == CounterValue(new_s, i)))
  && (result == SBI_SUCCESS && Bits(stop_flags, 1, 1) == 1 ==> (forall i: UInt | !CounterInSet(old_s, counter_idx_base, counter_idx_mask, i) ==> SnapshotShmemCounterValueUnchanged(new_s, i)))
  && (result == SBI_SUCCESS && Bits(stop_flags, 1, 1) == 1 ==> SnapshotShmemOverflowBitmapUpdated(new_s))
  && ((!(exists i: UInt | CounterInSet(old_s, counter_idx_base, counter_idx_mask, i) && !IsValidCounter(old_s, i)) &&
       !(Bits(stop_flags, XLEN - 1, 2) != 0) &&
       !(exists i: UInt | CounterInSet(old_s, counter_idx_base, counter_idx_mask, i) && CounterIsStopped(old_s, i)) &&
       !(Bits(stop_flags, 1, 1) == 1 && !SnapshotShmemAvailable(old_s)))
    ==> result == SBI_SUCCESS)
  && (result != SBI_SUCCESS
    ==> forall i: UInt | CounterInSet(old_s, counter_idx_base, counter_idx_mask, i) ==> CounterIsStopped(new_s, i))
  && (result != SBI_SUCCESS
    && Bits(stop_flags, 0, 0) == 1
    ==> (forall i: UInt | CounterInSet(old_s, counter_idx_base, counter_idx_mask, i) ==> CounterEventMappingIsReset(new_s, i)))
  && (result != SBI_SUCCESS
    && Bits(stop_flags, 1, 1) == 1
    ==> (forall i: UInt | CounterInSet(old_s, counter_idx_base, counter_idx_mask, i) ==> SnapshotShmemCounterValue(new_s, i) == CounterValue(new_s, i)))
  && (result != SBI_SUCCESS
    && Bits(stop_flags, 1, 1) == 1
    ==> (forall i: UInt | !CounterInSet(old_s, counter_idx_base, counter_idx_mask, i) ==> SnapshotShmemCounterValueUnchanged(new_s, i)))
  && (result != SBI_SUCCESS
    && Bits(stop_flags, 1, 1) == 1
    ==> SnapshotShmemOverflowBitmapUpdated(new_s))
}