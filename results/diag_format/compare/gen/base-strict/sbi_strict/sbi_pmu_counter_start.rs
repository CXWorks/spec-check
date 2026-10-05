pub open spec fn sbi_pmu_counter_start_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.is_err() ==> false)
    && (result.is_ok() ==> (
        (forall i: CounterIndex | InCounterSet(old_s.counter_idx_base, old_s.counter_idx_mask, i) ==> CounterStarted(new_s, i))
        && (Bits(old_s.start_flags, 0, 0) == 1 ==> (forall i: CounterIndex | InCounterSet(old_s.counter_idx_base, old_s.counter_idx_mask, i) ==> CounterValue(new_s, i) == old_s.initial_value))
        && (Bits(old_s.start_flags, 1, 1) == 1 ==> (forall i: CounterIndex | InCounterSet(old_s.counter_idx_base, old_s.counter_idx_mask, i) && IsValidCounter(i) ==> CounterValue(new_s, i) == SnapshotShmemCounterValue(old_s, i)))
        && (Bits(old_s.start_flags, 0, 0) == 0 && Bits(old_s.start_flags, 1, 1) == 0 ==> (forall i: CounterIndex | InCounterSet(old_s.counter_idx_base, old_s.counter_idx_mask, i) ==> CounterValue(new_s, i) == PreCounterValue(old_s, i)))
    ))
}