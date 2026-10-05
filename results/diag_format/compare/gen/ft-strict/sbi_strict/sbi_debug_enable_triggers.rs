pub open spec fn sbi_debug_enable_triggers_spec(trig_idx_base: UInt, trig_idx_mask: Bitmask, error: SbiErrorCode, old_s: S, new_s: S) -> bool {
  (exists|i: UInt| InTriggerSet(old_s, trig_idx_base, trig_idx_mask, i) && !IsMappedToHwTrigger(old_s, CallingHart(), i) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
  && (exists|i: UInt| InTriggerSet(old_s, trig_idx_base, trig_idx_mask, i) && i >= trig_max(old_s) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
  && (ResultEqual(error, SBI_SUCCESS) ==> ResultEqual(error, SBI_SUCCESS))
  && (ResultEqual(error, SBI_SUCCESS) ==> forall|i: UInt| InTriggerSet(old_s, trig_idx_base, trig_idx_mask, i) ==> MappedHwTrigger(new_s, CallingHart(), i).vs == TrigState(new_s, CallingHart(), i).vs)
  && (ResultEqual(error, SBI_SUCCESS) ==> forall|i: UInt| InTriggerSet(old_s, trig_idx_base, trig_idx_mask, i) ==> MappedHwTrigger(new_s, CallingHart(), i).vu == TrigState(new_s, CallingHart(), i).vu)
  && (ResultEqual(error, SBI_SUCCESS) ==> forall|i: UInt| InTriggerSet(old_s, trig_idx_base, trig_idx_mask, i) ==> MappedHwTrigger(new_s, CallingHart(), i).s == TrigState(new_s, CallingHart(), i).s)
  && (ResultEqual(error, SBI_SUCCESS) ==> forall|i: UInt| InTriggerSet(old_s, trig_idx_base, trig_idx_mask, i) ==> MappedHwTrigger(new_s, CallingHart(), i).u == TrigState(new_s, CallingHart(), i).u)
  && ((!(exists|i: UInt| InTriggerSet(old_s, trig_idx_base, trig_idx_mask, i) && !IsMappedToHwTrigger(old_s, CallingHart(), i)) &&
       !(exists|i: UInt| InTriggerSet(old_s, trig_idx_base, trig_idx_mask, i) && i >= trig_max(old_s)))
    ==> ResultEqual(error, SBI_SUCCESS))
  && (result != SBI_SUCCESS
    ==> MappedHwTrigger(new_s, CallingHart(), 0).vs == MappedHwTrigger(old_s, CallingHart(), 0).vs)
  && (result != SBI_SUCCESS
    ==> MappedHwTrigger(new_s, CallingHart(), 0).vu == MappedHwTrigger(old_s, CallingHart(), 0).vu)
  && (result != SBI_SUCCESS
    ==> MappedHwTrigger(new_s, CallingHart(), 0).s == MappedHwTrigger(old_s, CallingHart(), 0).s)
  && (result != SBI_SUCCESS
    ==> MappedHwTrigger(new_s, CallingHart(), 0).u == MappedHwTrigger(old_s, CallingHart(), 0).u)
}