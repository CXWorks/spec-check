pub open spec fn sbi_debug_disable_triggers_spec(trig_idx_base: unsigned long, trig_idx_mask: unsigned long, result: sbiret, old_s: S, new_s: S) -> bool {
  (Exists(trig_idx in TriggerSet(old_s, trig_idx_base, trig_idx_mask) : !IsMappedToHwTrigger(old_s, trig_idx)) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (Exists(trig_idx in TriggerSet(old_s, trig_idx_base, trig_idx_mask) : trig_idx >= trig_max) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (result == SBI_SUCCESS ==> ResultEqual(result, SBI_SUCCESS))
  && (result == SBI_SUCCESS ==> ForAll(trig_idx in TriggerSet(old_s, trig_idx_base, trig_idx_mask) : HwTriggerOf(new_s, trig_idx).vs == 0 && HwTriggerOf(new_s, trig_idx).vu == 0 && HwTriggerOf(new_s, trig_idx).s == 0 && HwTriggerOf(new_s, trig_idx).u == 0))
  && ((!(Exists(trig_idx in TriggerSet(old_s, trig_idx_base, trig_idx_mask) : !IsMappedToHwTrigger(old_s, trig_idx))) &&
       !(Exists(trig_idx in TriggerSet(old_s, trig_idx_base, trig_idx_mask) : trig_idx >= trig_max)))
    ==> result == SBI_SUCCESS)
  && (result != SBI_SUCCESS
    ==> ForAll(trig_idx in TriggerSet(old_s, trig_idx_base, trig_idx_mask) : HwTriggerOf(new_s, trig_idx).vs == HwTriggerOf(old_s, trig_idx).vs &&
                                            HwTriggerOf(new_s, trig_idx).vu == HwTriggerOf(old_s, trig_idx).vu &&
                                            HwTriggerOf(new_s, trig_idx).s == HwTriggerOf(old_s, trig_idx).s &&
                                            HwTriggerOf(new_s, trig_idx).u == HwTriggerOf(old_s, trig_idx).u))
}