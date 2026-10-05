pub open spec fn sbi_pmu_counter_config_matching_spec(counter_idx_base: u64, counter_idx_mask: u64, config_flags: u64, event_idx: u64, event_data: u64, ret: SbiRet, old_s: S, new_s: S) -> bool {
    ((((config_flags >> 8u64) != 0u64) || !CounterSetValid(old_s, counter_idx_base, counter_idx_mask)) ==> ret.error == SBI_ERR_INVALID_PARAM)
    && ((((config_flags >> 8u64) == 0u64) && CounterSetValid(old_s, counter_idx_base, counter_idx_mask) && (config_flags & 0x1u64) == 0u64 && !ExistsMatchingCounter(old_s, counter_idx_base, counter_idx_mask, event_idx, event_data)) ==> ret.error == SBI_ERR_NOT_SUPPORTED)
    && (ret.error != SBI_SUCCESS ==> new_s == old_s)
    && (ret.error == SBI_SUCCESS ==> (
        ((config_flags >> 8u64) == 0u64)
        && CounterSetValid(old_s, counter_idx_base, counter_idx_mask)
        && CounterInSet(counter_idx_base, counter_idx_mask, ret.value)
        && (((config_flags & 0x1u64) != 0u64) ==> ret.value == FirstCounterInSet(counter_idx_base, counter_idx_mask))
        && (((config_flags & 0x1u64) == 0u64) ==> (!CounterStarted(old_s, ret.value) && CounterCanMonitorEvent(old_s, ret.value, event_idx, event_data)))
        && CounterConfiguredForEvent(new_s, ret.value, event_idx, event_data)
        && (((config_flags & 0x2u64) != 0u64) ==> CounterValue(new_s, ret.value) == 0u64)
        && (((config_flags & 0x2u64) == 0u64) ==> CounterValue(new_s, ret.value) == CounterValue(old_s, ret.value))
        && (((config_flags & 0x4u64) != 0u64) ==> CounterStarted(new_s, ret.value))
    ))
}
