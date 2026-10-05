pub open spec fn sbi_debug_enable_triggers_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.error == SBI_ERR_INVALID_PARAM ==> (
        (exists (trig_idx: u64),
         trig_idx >= old_s.trig_max
         && (trig_idx as u64) >= (old_s.trig_idx_base as u64)
         && (trig_idx as u64) < (old_s.trig_idx_base as u64) + (old_s.trig_idx_mask as u64))
        ||
        (exists (trig_idx: u64),
         (trig_idx as u64) >= (old_s.trig_idx_base as u64)
         && (trig_idx as u64) < (old_s.trig_idx_base as u64) + (old_s.trig_idx_mask as u64)
         && !old_s.trig_mapped(trig_idx))
    ))
    && (result.error == SBI_SUCCESS ==> (
        (forall (trig_idx: u64),
         (trig_idx as u64) >= (old_s.trig_idx_base as u64)
         && (trig_idx as u64) < (old_s.trig_idx_base as u64) + (old_s.trig_idx_mask as u64)
         ==> (
             new_s.trig_mapped(trig_idx)
             && new_s.trig_state(trig_idx) == old_s.trig_state(trig_idx)
         ))
    ))
}