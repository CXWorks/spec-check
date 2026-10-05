pub open spec fn sbi_pmu_counter_config_matching_spec(counter_idx_base: unsigned long, counter_idx_mask: unsigned long, config_flags: unsigned long, event_idx: unsigned long, event_data: uint64_t, result: struct sbiret, old_s: S, new_s: S) -> bool {
  (result.code == 0 ==> CounterInSet(new_s, selected_counter(new_s), counter_idx_base, counter_idx_mask))
  && (result.code == 0 && !FlagSet(old_s, config_flags, SBI_PMU_CFG_FLAG_SKIP_MATCH) ==> !CounterStarted'(new_s, selected_counter(new_s)) && CounterCanMonitor(new_s, selected_counter(new_s), event_idx))
  && (result.code == 0 && FlagSet(old_s, config_flags, SBI_PMU_CFG_FLAG_SKIP_MATCH) ==> selected_counter(new_s) == FirstCounterInSet(new_s, counter_idx_base, counter_idx_mask))
  && (result.code == 0 ==> CounterEvent(new_s, selected_counter(new_s)) == event_idx && CounterEventData(new_s, selected_counter(new_s)) == event_data)
  && (result.code == 0 && FlagSet(old_s, config_flags, SBI_PMU_CFG_FLAG_CLEAR_VALUE) ==> CounterValue(new_s, selected_counter(new_s)) == 0)
  && (result.code == 0 && FlagSet(old_s, config_flags, SBI_PMU_CFG_FLAG_AUTO_START) ==> CounterStarted(new_s, selected_counter(new_s)))
  && ((!(result.code == 0) || !(result.code == 0 && !FlagSet(old_s, config_flags, SBI_PMU_CFG_FLAG_SKIP_MATCH)) || !(result.code == 0 && FlagSet(old_s, config_flags, SBI_PMU_CFG_FLAG_SKIP_MATCH)) || !(result.code == 0) || !(result.code == 0 && FlagSet(old_s, config_flags, SBI_PMU_CFG_FLAG_CLEAR_VALUE)) || !(result.code == 0 && FlagSet(old_s, config_flags, SBI_PMU_CFG_FLAG_AUTO_START)))
    ==> CounterStarted(new_s, selected_counter(new_s)) == CounterStarted(old_s, selected_counter(old_s)))
  && (result.code != 0
    ==> CounterEvent(new_s, selected_counter(new_s)) == CounterEvent(old_s, selected_counter(old_s)))
  && (result.code != 0
    ==> CounterEventData(new_s, selected_counter(new_s)) == CounterEventData(old_s, selected_counter(old_s)))
  && (result.code != 0
    ==> CounterValue(new_s, selected_counter(new_s)) == CounterValue(old_s, selected_counter(old_s)))
  && (result.code != 0
    ==> CounterStarted(new_s, selected_counter(new_s)) == CounterStarted(old_s, selected_counter(old_s)))
}