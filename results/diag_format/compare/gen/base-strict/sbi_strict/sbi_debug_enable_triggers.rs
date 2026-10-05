pub open spec fn sbi_debug_enable_triggers_spec(error: SbiErrorCode, old_s: S, new_s: S) -> bool {
    (exists|i: UInt| InTriggerSet(trig_idx_base, trig_idx_mask, i) && !IsMappedToHwTrigger(CallingHart(), i) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
    && (exists|i: UInt| InTriggerSet(trig_idx_base, trig_idx_mask, i) && i >= trig_max ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
    && (ResultEqual(error, SBI_SUCCESS) ==> (forall|i: UInt| InTriggerSet(trig_idx_base, trig_idx_mask, i) ==> (MappedHwTrigger(CallingHart(), i).vs == TrigState(CallingHart(), i).vs && MappedHwTrigger(CallingHart(), i).vu == TrigState(CallingHart(), i).vu && MappedHwTrigger(CallingHart(), i).s == TrigState(CallingHart(), i).s && MappedHwTrigger(CallingHart(), i).u == TrigState(CallingHart(), i).u)))
}