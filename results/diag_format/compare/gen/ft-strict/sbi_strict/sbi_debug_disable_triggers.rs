pub open spec fn sbi_debug_disable_triggers_spec(trig_idx_base: UInt64, trig_idx_mask: UInt64, result: long, old_s: S, new_s: S) -> bool {
  (exists|trig_idx: UInt64| TriggerInSet(old_s, trig_idx_base, trig_idx_mask, trig_idx) && (!IsMappedToHwTrigger(old_s, trig_idx) || trig_idx >= TrigMax(old_s)) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (ResultEqual(result, SBI_SUCCESS) ==> ResultEqual(result, SBI_SUCCESS))
  && (ResultEqual(result, SBI_SUCCESS) ==> forall|trig_idx: UInt64| TriggerInSet(old_s, trig_idx_base, trig_idx_mask, trig_idx) ==> MappedHwTrigger(new_s, trig_idx).vs == 0)
  && (ResultEqual(result, SBI_SUCCESS) ==> forall|trig_idx: UInt64| TriggerInSet(old_s, trig_idx_base, trig_idx_mask, trig_idx) ==> MappedHwTrigger(new_s, trig_idx).vu == 0)
  && (ResultEqual(result, SBI_SUCCESS) ==> forall|trig_idx: UInt64| TriggerInSet(old_s, trig_idx_base, trig_idx_mask, trig_idx) ==> MappedHwTrigger(new_s, trig_idx).s == 0)
  && (ResultEqual(result, SBI_SUCCESS) ==> forall|trig_idx: UInt64| TriggerInSet(old_s, trig_idx_base, trig_idx_mask, trig_idx) ==> MappedHwTrigger(new_s, trig_idx).u == 0)
  && ((!(exists|trig_idx: UInt64| TriggerInSet(old_s, trig_idx_base, trig_idx_mask, trig_idx) && (!IsMappedToHwTrigger(old_s, trig_idx) || trig_idx >= TrigMax(old_s))))
    ==> ResultEqual(result, SBI_SUCCESS))
}