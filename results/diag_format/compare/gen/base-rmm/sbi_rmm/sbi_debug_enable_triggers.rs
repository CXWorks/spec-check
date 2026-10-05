pub open spec fn sbi_debug_enable_triggers_spec(result: sbiret, trig_idx_base: UInt, trig_idx_mask: UInt, old_s: S, new_s: S) -> bool {
    (Exists(trig_idx in TrigSet(trig_idx_base, trig_idx_mask) : (trig_idx as UInt) >= trig_max) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (Exists(trig_idx in TrigSet(trig_idx_base, trig_idx_mask) : !IsMappedToHwTrigger(trig_idx)) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (ResultEqual(result, SBI_SUCCESS) ==> (
        ForAll(trig_idx in TrigSet(trig_idx_base, trig_idx_mask) : (MappedHwTrigger(new_s, trig_idx).vs == trig_state(new_s, trig_idx).saved.vs))
        && ForAll(trig_idx in TrigSet(trig_idx_base, trig_idx_mask) : (MappedHwTrigger(new_s, trig_idx).vu == trig_state(new_s, trig_idx).saved.vu))
        && ForAll(trig_idx in TrigSet(trig_idx_base, trig_idx_mask) : (MappedHwTrigger(new_s, trig_idx).s == trig_state(new_s, trig_idx).saved.s))
        && ForAll(trig_idx in TrigSet(trig_idx_base, trig_idx_mask) : (MappedHwTrigger(new_s, trig_idx).u == trig_state(new_s, trig_idx).saved.u))
    ))
}