pub open spec fn sbi_debug_uninstall_triggers_spec(trig_idx_base: unsigned long, trig_idx_mask: unsigned long, result: sbiret, old_s: S, new_s: S) -> bool {
  (Exists(trig_idx in TriggerSet(old_s, trig_idx_base, trig_idx_mask) : !IsMappedToHwTrigger(old_s, trig_idx)) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (Exists(trig_idx in TriggerSet(old_s, trig_idx_base, trig_idx_mask) : trig_idx >= trig_max) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (result == SBI_SUCCESS ==> ResultEqual(result, SBI_SUCCESS))
  && (result == SBI_SUCCESS ==> ForAll(trig_idx in TriggerSet(old_s, trig_idx_base, trig_idx_mask) : HwTriggerOf(new_s, trig_idx).tdata1 == 0 && HwTriggerOf(new_s, trig_idx).tdata2 == 0 && HwTriggerOf(new_s, trig_idx).tdata3 == 0))
  && (result == SBI_SUCCESS ==> ForAll(trig_idx in TriggerSet(old_s, trig_idx_base, trig_idx_mask) : TriggerAt(new_s, trig_idx).trig_state == CLEARED))
  && (result == SBI_SUCCESS ==> ForAll(trig_idx in TriggerSet(old_s, trig_idx_base, trig_idx_mask) : !IsMappedToHwTrigger(new_s, trig_idx) && IsFreeTrigIdx(new_s, trig_idx) && IsFreeHwTrigger(new_s, HwTriggerOf(new_s, trig_idx))))
  && ((!(Exists(trig_idx in TriggerSet(old_s, trig_idx_base, trig_idx_mask) : !IsMappedToHwTrigger(old_s, trig_idx))) &&
       !(Exists(trig_idx in TriggerSet(old_s, trig_idx_base, trig_idx_mask) : trig_idx >= trig_max)))
    ==> result == SBI_SUCCESS)
  && (result != SBI_SUCCESS
    ==> ForAll(trig_idx in TriggerSet(old_s, trig_idx_base, trig_idx_mask) : HwTriggerOf(new_s, trig_idx).tdata1 == HwTriggerOf(old_s, trig_idx).tdata1 && HwTriggerOf(new_s, trig_idx).tdata2 == HwTriggerOf(old_s, trig_idx).tdata2 && HwTriggerOf(new_s, trig_idx).tdata3 == HwTriggerOf(old_s, trig_idx).tdata3))
  && (result != SBI_SUCCESS
    ==> ForAll(trig_idx in TriggerSet(old_s, trig_idx_base, trig_idx_mask) : TriggerAt(new_s, trig_idx).trig_state == TriggerAt(old_s, trig_idx).trig_state))
  && (result != SBI_SUCCESS
    ==> ForAll(trig_idx in TriggerSet(old_s, trig_idx_base, trig_idx_mask) : IsMappedToHwTrigger(new_s, trig_idx) == IsMappedToHwTrigger(old_s, trig_idx)))
  && (result != SBI_SUCCESS
    ==> ForAll(trig_idx in TriggerSet(old_s, trig_idx_base, trig_idx_mask) : IsFreeTrigIdx(new_s, trig_idx) == IsFreeTrigIdx(old_s, trig_idx)))
  && (result != SBI_SUCCESS
    ==> ForAll(trig_idx in TriggerSet(old_s, trig_idx_base, trig_idx_mask) : IsFreeHwTrigger(new_s, HwTriggerOf(new_s, trig_idx)) == IsFreeHwTrigger(old_s, HwTriggerOf(old_s, trig_idx))))
}