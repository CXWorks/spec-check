pub open spec fn sbi_debug_disable_triggers_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.error == SBI_ERR_INVALID_PARAM ==> (
        (forall i: u64 :: i < (trig_idx_mask as u64) as u64 ==>
            (old_s.debug_triggers[trig_idx_base as u64 + i].vs == 0 &&
             old_s.debug_triggers[trig_idx_base as u64 + i].vu == 0 &&
             old_s.debug_triggers[trig_idx_base as u64 + i].s == 0 &&
             old_s.debug_triggers[trig_idx_base as u64 + i].u == 0 &&
             (old_s.debug_triggers[trig_idx_base as u64 + i].mapped == false ||
              (trig_idx_base as u64 + i) >= old_s.trig_max)))
    ))
    && (result.error == SBI_SUCCESS ==> (
        (forall i: u64 :: i < (trig_idx_mask as u64) as u64 ==>
            (new_s.debug_triggers[trig_idx_base as u64 + i].vs == 0 &&
             new_s.debug_triggers[trig_idx_base as u64 + i].vu == 0 &&
             new_s.debug_triggers[trig_idx_base as u64 + i].s == 0 &&
             new_s.debug_triggers[trig_idx_base as u64 + i].u == 0))
    ))
    && (result.error != SBI_ERR_INVALID_PARAM && result.error != SBI_SUCCESS ==> true)
}