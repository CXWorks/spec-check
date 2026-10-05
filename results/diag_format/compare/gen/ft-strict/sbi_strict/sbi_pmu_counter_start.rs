pub open spec fn sbi_pmu_counter_start_spec(counter_idx_base: unsigned long, counter_idx_mask: unsigned long, start_flags: unsigned long, initial_value: uint64_t, result: SbiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == SBI_SUCCESS ==> (forall i: CounterIndex| InCounterSet(counter_idx_base, counter_idx_mask, i) ==> CounterStarted(new_s, i)))
  && (result == SBI_SUCCESS && (Bits(start_flags, 0, 0) == 1) ==> (forall i: CounterIndex| InCounterSet(counter_idx_base, counter_idx_mask, i) ==> CounterValue(new_s, i) == initial_value))
  && (result == SBI_SUCCESS && (Bits(start_flags, 1, 1) == 1) ==> (forall i: CounterIndex| (InCounterSet(counter_idx_base, counter_idx_mask, i) && IsValidCounter(new_s, i)) ==> CounterValue(new_s, i) == SnapshotShmemCounterValue(new_s, i)))
  && (result == SBI_SUCCESS && (Bits(start_flags, 0, 0) == 0 && Bits(start_flags, 1, 1) == 0) ==> (forall i: CounterIndex| InCounterSet(counter_idx_base, counter_idx_mask, i) ==> CounterValue(new_s, i) == PreCounterValue(new_s, i)))
  && ((!(result == SBI_SUCCESS))
    ==> (forall i: CounterIndex| InCounterSet(counter_idx_base, counter_idx_mask, i) ==> !(CounterStarted(new_s, i))))
  && ((!(result == SBI_SUCCESS))
    ==> (forall i: CounterIndex| InCounterSet(counter_idx_base, counter_idx_mask, i) ==> CounterValue(new_s, i) == CounterValue(old_s, i)))
  && ((!(result == SBI_SUCCESS))
    ==> (forall i: CounterIndex| (InCounterSet(counter_idx_base, counter_idx_mask, i) && IsValidCounter(old_s, i)) ==> CounterValue(new_s, i) == CounterValue(old_s, i)))
  && ((!(result == SBI_SUCCESS))
    ==> (forall i: CounterIndex| InCounterSet(counter_idx_base, counter_idx_mask, i) ==> CounterValue(new_s, i) == CounterValue(old_s, i)))
}