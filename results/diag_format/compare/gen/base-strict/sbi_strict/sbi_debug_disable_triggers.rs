pub open spec fn sbi_debug_disable_triggers_spec(result: long, old_s: S, new_s: S) -> bool {
    (exists|trig_idx: UInt64| TriggerInSet(old_s, trig_idx_base, trig_idx_mask, trig_idx) && (!IsMappedToHwTrigger(trig_idx) || trig_idx >= TrigMax()) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (ResultEqual(result, SBI_SUCCESS) ==> (forall|trig_idx: UInt64| TriggerInSet(old_s, trig_idx_base, trig_idx_mask, trig_idx) ==> (MappedHwTrigger(trig_idx).vs == 0 && MappedHwTrigger(trig_idx).vu == 0 && MappedHwTrigger(trig_idx).s == 0 && MappedHwTrigger(trig_idx).u == 0)))
}