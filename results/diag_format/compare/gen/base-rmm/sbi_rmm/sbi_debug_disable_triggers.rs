pub open spec fn sbi_debug_disable_triggers_spec(result: sbiret, trig_idx_base: UInt, trig_idx_mask: UInt, old_s: S, new_s: S) -> bool {
    (Exists(trig_idx in TriggerSet(trig_idx_base, trig_idx_mask) : !IsMappedToHwTrigger(trig_idx)) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (Exists(trig_idx in TriggerSet(trig_idx_base, trig_idx_mask) : trig_idx >= trig_max) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (ResultEqual(result, SBI_SUCCESS) ==> ForAll(trig_idx in TriggerSet(trig_idx_base, trig_idx_mask) : HwTriggerOf(trig_idx).vs == 0 && HwTriggerOf(trig_idx).vu == 0 && HwTriggerOf(trig_idx).s == 0 && HwTriggerOf(trig_idx).u == 0))
}