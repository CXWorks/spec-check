pub open spec fn sbi_debug_uninstall_triggers_spec(trig_idx_base: UInt64, trig_idx_mask: Bits64, result: SbiErrorCode, old_s: S, new_s: S) -> bool {
  (exists|i: UInt64| TriggerInSet(old_s, trig_idx_base, trig_idx_mask, i) && !IsMappedToHwTrigger(old_s, i) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (exists|i: UInt64| TriggerInSet(old_s, trig_idx_base, trig_idx_mask, i) && i >= trig_max(old_s) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (result == SBI_SUCCESS ==> ResultEqual(result, SBI_SUCCESS))
  && (result == SBI_SUCCESS ==> forall|i: UInt64| TriggerInSet(old_s, trig_idx_base, trig_idx_mask, i) ==> PreMappedHwTrigger(new_s, i).tdata1 == 0)
  && (result == SBI_SUCCESS ==> forall|i: UInt64| TriggerInSet(old_s, trig_idx_base, trig_idx_mask, i) ==> PreMappedHwTrigger(new_s, i).tdata2 == 0)
  && (result == SBI_SUCCESS ==> forall|i: UInt64| TriggerInSet(old_s, trig_idx_base, trig_idx_mask, i) ==> PreMappedHwTrigger(new_s, i).tdata3 == 0)
  && (result == SBI_SUCCESS ==> forall|i: UInt64| TriggerInSet(old_s, trig_idx_base, trig_idx_mask, i) ==> DebugTrigger(new_s, i).trig_state == 0)
  && (result == SBI_SUCCESS ==> forall|i: UInt64| TriggerInSet(old_s, trig_idx_base, trig_idx_mask, i) ==> !IsMappedToHwTrigger(new_s, i))
  && (result == SBI_SUCCESS ==> forall|i: UInt64| TriggerInSet(old_s, trig_idx_base, trig_idx_mask, i) ==> IsHwTriggerFree(new_s, PreMappedHwTrigger(new_s, i)))
  && (result == SBI_SUCCESS ==> forall|i: UInt64| TriggerInSet(old_s, trig_idx_base, trig_idx_mask, i) ==> IsTrigIdxFree(new_s, i))
  && ((!(exists|i: UInt64| TriggerInSet(old_s, trig_idx_base, trig_idx_mask, i) && !IsMappedToHwTrigger(old_s, i)) &&
       !(exists|i: UInt64| TriggerInSet(old_s, trig_idx_base, trig_idx_mask, i) && i >= trig_max(old_s)))
    ==> result == SBI_SUCCESS)
  && (result != SBI_SUCCESS
    ==> forall|i: UInt64| TriggerInSet(old_s, trig_idx_base, trig_idx_mask, i) ==> PreMappedHwTrigger(new_s, i).tdata1 == PreMappedHwTrigger(old_s, i).tdata1)
  && (result != SBI_SUCCESS
    ==> forall|i: UInt64| TriggerInSet(old_s, trig_idx_base, trig_idx_mask, i) ==> PreMappedHwTrigger(new_s, i).tdata2 == PreMappedHwTrigger(old_s, i).tdata2)
  && (result != SBI_SUCCESS
    ==> forall|i: UInt64| TriggerInSet(old_s, trig_idx_base, trig_idx_mask, i) ==> PreMappedHwTrigger(new_s, i).tdata3 == PreMappedHwTrigger(old_s, i).tdata3)
  && (result != SBI_SUCCESS
    ==> forall|i: UInt64| TriggerInSet(old_s, trig_idx_base, trig_idx_mask, i) ==> DebugTrigger(new_s, i).trig_state == DebugTrigger(old_s, i).trig_state)
  && (result != SBI_SUCCESS
    ==> forall|i: UInt64| TriggerInSet(old_s, trig_idx_base, trig_idx_mask, i) ==> IsMappedToHwTrigger(new_s, i) == IsMappedToHwTrigger(old_s, i))
  && (result != SBI_SUCCESS
    ==> forall|i: UInt64| TriggerInSet(old_s, trig_idx_base, trig_idx_mask, i) ==> !IsHwTriggerFree(new_s, PreMappedHwTrigger(new_s, i)) == !IsHwTriggerFree(old_s, PreMappedHwTrigger(old_s, i)))
  && (result != SBI_SUCCESS
    ==> forall|i: UInt64| TriggerInSet(old_s, trig_idx_base, trig_idx_mask, i) ==> !IsTrigIdxFree(new_s, i) == !IsTrigIdxFree(old_s, i))
}