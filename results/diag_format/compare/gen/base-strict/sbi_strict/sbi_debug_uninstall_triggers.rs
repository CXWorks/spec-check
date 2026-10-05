pub open spec fn sbi_debug_uninstall_triggers_spec(result: SbiErrorCode, old_s: S, new_s: S, trig_idx_base: UInt64, trig_idx_mask: Bits64) -> bool {
    (exists i: UInt64 | TriggerInSet(trig_idx_base, trig_idx_mask, i) && !IsMappedToHwTrigger(i) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (exists i: UInt64 | TriggerInSet(trig_idx_base, trig_idx_mask, i) && i >= trig_max ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (ResultEqual(result, SBI_SUCCESS) ==> (forall i: UInt64 | TriggerInSet(trig_idx_base, trig_idx_mask, i) ==> (PreMappedHwTrigger(i).tdata1 == 0 && PreMappedHwTrigger(i).tdata2 == 0 && PreMappedHwTrigger(i).tdata3 == 0 && DebugTrigger(i).trig_state == 0 && !IsMappedToHwTrigger(i) && IsHwTriggerFree(PreMappedHwTrigger(i)) && IsTrigIdxFree(i))))
}