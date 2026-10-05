pub open spec fn sbi_pmu_counter_config_matching_spec(counter_idx_base: unsigned long, counter_idx_mask: unsigned long, config_flags: unsigned long, event_idx: unsigned long, event_data: uint64_t, result: SbiRet, old_s: S, new_s: S) -> bool {
  (result.error == 0 ==> IsCounterInSet(new_s, SelectedCounter(new_s), counter_idx_base, counter_idx_mask))
  && (result.error == 0 && (config_flags & 1) == 1 ==> SelectedCounter(new_s) == FirstCounterInSet(new_s, counter_idx_base, counter_idx_mask))
  && (result.error == 0 && (config_flags & 1) == 0 ==> !CounterWasStarted(new_s, SelectedCounter(new_s)))
  && (result.error == 0 && (config_flags & 1) == 0 ==> CounterCanMonitorEvent(new_s, SelectedCounter(new_s), event_idx))
  && (result.error == 0 ==> CounterConfiguredForEvent(new_s, SelectedCounter(new_s), event_idx, event_data))
  && (result.error == 0 && (config_flags & 2) == 1 ==> CounterValue(new_s, SelectedCounter(new_s)) == 0)
  && (result.error == 0 && (config_flags & 4) == 1 ==> CounterIsStarted(new_s, SelectedCounter(new_s)))
  && (result.error == 0 && (config_flags & 4) == 1 ==> CounterValueUnaffectedByAutoStart(new_s, SelectedCounter(new_s)))
  && ((!(result.error == 0) ||
       !(IsCounterInSet(new_s, SelectedCounter(new_s), counter_idx_base, counter_idx_mask)) &&
       !((config_flags & 1) == 1 && SelectedCounter(new_s) == FirstCounterInSet(new_s, counter_idx_base, counter_idx_mask)) &&
       !((config_flags & 1) == 0 && !CounterWasStarted(new_s, SelectedCounter(new_s))) &&
       !((config_flags & 1) == 0 && CounterCanMonitorEvent(new_s, SelectedCounter(new_s), event_idx)) &&
       !CounterConfiguredForEvent(new_s, SelectedCounter(new_s), event_idx, event_data) &&
       !((config_flags & 2) == 1 && CounterValue(new_s, SelectedCounter(new_s)) == 0) &&
       !((config_flags & 4) == 1 && CounterIsStarted(new_s, SelectedCounter(new_s))) &&
       !((config_flags & 4) == 1 && CounterValueUnaffectedByAutoStart(new_s, SelectedCounter(new_s))))
    ==> result.error != 0)
}