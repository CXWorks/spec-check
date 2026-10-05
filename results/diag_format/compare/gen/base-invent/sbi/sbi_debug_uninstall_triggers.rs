pub open spec fn sbi_debug_uninstall_triggers_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.error == SBI_ERR_INVALID_PARAM ==> (
        (forall i: u64 where i < 64 ::
            (trig_idx_base as u64 + (i as u64) & trig_idx_mask as u64) != 0 ==>
            (
                (old_s.trig_state((trig_idx_base as u64 + (i as u64) & trig_idx_mask as u64)) == TRIG_STATE_MAPPED) ==>
                (new_s.trig_state((trig_idx_base as u64 + (i as u64) & trig_idx_mask as u64)) == TRIG_STATE_UNMAPPED)
            ) ||
                (old_s.trig_state((trig_idx_base as u64 + (i as u64) & trig_idx_mask as u64)) == TRIG_STATE_UNMAPPED)
            ||
                (trig_idx_base as u64 + (i as u64) & trig_idx_mask as u64) >= trig_max
        ))
    ))
    && (result.error == SBI_SUCCESS ==> (
        (forall i: u64 where i < 64 ::
            (trig_idx_base as u64 + (i as u64) & trig_idx_mask as u64) != 0 ==>
            (
                (old_s.trig_state((trig_idx_base as u64 + (i as u64) & trig_idx_mask as u64)) == TRIG_STATE_MAPPED) ==>
                (new_s.trig_state((trig_idx_base as u64 + (i as u64) & trig_idx_mask as u64)) == TRIG_STATE_UNMAPPED)
            )
        ))
    ))
    && (result.error == SBI_SUCCESS ==> (
        (forall i: u64 where i < 64 ::
            (trig_idx_base as u64 + (i as u64) & trig_idx_mask as u64) != 0 ==>
            (
                (old_s.trig_state((trig_idx_base as u64 + (i as u64) & trig_idx_mask as u64)) == TRIG_STATE_MAPPED) ==>
                (new_s.tdata1((trig_idx_base as u64 + (i as u64) & trig_idx_mask as u64)) == 0)
            )
        ))
    ))
    && (result.error == SBI_SUCCESS ==> (
        (forall i: u64 where i < 64 ::
            (trig_idx_base as u64 + (i as u64) & trig_idx_mask as u64) != 0 ==>
            (
                (old_s.trig_state((trig_idx_base as u64 + (i as u64) & trig_idx_mask as u64)) == TRIG_STATE_MAPPED) ==>
                (new_s.tdata2((trig_idx_base as u64 + (i as u64) & trig_idx_mask as u64)) == 0)
            )
        ))
    ))
    && (result.error == SBI_SUCCESS ==> (
        (forall i: u64 where i < 64 ::
            (trig_idx_base as u64 + (i as u64) & trig_idx_mask as u64) != 0 ==>
            (
                (old_s.trig_state((trig_idx_base as u64 + (i as u64) & trig_idx_mask as u64)) == TRIG_STATE_MAPPED) ==>
                (new_s.tdata3((trig_idx_base as u64 + (i as u64) & trig_idx_mask as u64)) == 0)
            )
        ))
    ))
}