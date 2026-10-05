pub open spec fn sbi_pmu_counter_start_spec(counter_idx_base: unsigned long, counter_idx_mask: unsigned long, start_flags: unsigned long, initial_value: uint64_t, result: SbiCommandReturnCode, old_s: S, new_s: S) -> bool {
  ((start_flags & SBI_PMU_START_SET_INIT_VALUE) != 0 && (start_flags & SBI_PMU_START_FLAG_INIT_SNAPSHOT) != 0 ==> RsiCommandReturnCode::RSI_ERROR_INPUT)
  && ((!(start_flags & SBI_PMU_START_SET_INIT_VALUE) != 0 && !(start_flags & SBI_PMU_START_FLAG_INIT_SNAPSHOT) != 0) ==> (forall idx in CounterSet(new_s, counter_idx_base, counter_idx_mask): CounterIsStarted(new_s, idx)))
  && ((start_flags & SBI_PMU_START_SET_INIT_VALUE) != 0 ==> (forall idx in CounterSet(new_s, counter_idx_base, counter_idx_mask): CounterValue(new_s, idx) == initial_value))
  && ((start_flags & SBI_PMU_START_FLAG_INIT_SNAPSHOT) != 0 ==> (forall idx in ValidCounters(new_s, CounterSet(new_s, counter_idx_base, counter_idx_mask)): CounterValue(new_s, idx) == SnapshotShmemCounterValue(new_s, idx)))
  && ((start_flags & SBI_PMU_START_SET_INIT_VALUE) == 0 && (start_flags & SBI_PMU_START_FLAG_INIT_SNAPSHOT) == 0 ==> (forall idx in CounterSet(new_s, counter_idx_base, counter_idx_mask): CounterValue(new_s, idx) == CounterValue(old_s, idx)))
  && ((result == RSI_SUCCESS) ==> (forall idx in CounterSet(new_s, counter_idx_base, counter_idx_mask): CounterIsStarted(new_s, idx)))
  && ((result == RSI_SUCCESS && (start_flags & SBI_PMU_START_SET_INIT_VALUE) != 0) ==> (forall idx in CounterSet(new_s, counter_idx_base, counter_idx_mask): CounterValue(new_s, idx) == initial_value))
  && ((result == RSI_SUCCESS && (start_flags & SBI_PMU_START_FLAG_INIT_SNAPSHOT) != 0) ==> (forall idx in ValidCounters(new_s, CounterSet(new_s, counter_idx_base, counter_idx_mask)): CounterValue(new_s, idx) == SnapshotShmemCounterValue(new_s, idx)))
  && ((result == RSI_SUCCESS && (start_flags & SBI_PMU_START_SET_INIT_VALUE) == 0 && (start_flags & SBI_PMU_START_FLAG_INIT_SNAPSHOT) == 0) ==> (forall idx in CounterSet(new_s, counter_idx_base, counter_idx_mask): CounterValue(new_s, idx) == CounterValue(old_s, idx)))
  && ((result != RSI_SUCCESS)
    ==> (forall idx in CounterSet(new_s, counter_idx_base, counter_idx_mask): CounterIsStarted(new_s, idx) == CounterIsStarted(old_s, idx)))
  && ((result != RSI_SUCCESS)
    ==> (forall idx in CounterSet(new_s, counter_idx_base, counter_idx_mask): CounterValue(new_s, idx) == CounterValue(old_s, idx)))
  && ((result != RSI_SUCCESS)
    ==> (forall idx in ValidCounters(new_s, CounterSet(new_s, counter_idx_base, counter_idx_mask)): CounterValue(new_s, idx) == CounterValue(old_s, idx)))
  && (result == RSI_SUCCESS
    ==> CounterValue(new_s, 0) == CounterValue(old_s, 0))
}