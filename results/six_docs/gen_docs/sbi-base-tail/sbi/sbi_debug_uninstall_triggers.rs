pub open spec fn sbi_debug_uninstall_triggers_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_INVALID_PARAM ==> (
        (forall i: int ::
            (i >= 0 as int) &&
            ((old_s.trig_idx_base as int) + i < (old_s.trig_idx_mask as int)) &&
            ((old_s.trig_idx_base as int) + i < (old_s.trig_max as int)) ==>
            (
                (old_s.trig_mapped(i) == false) ||
                (old_s.trig_state(i) != 0 as int)
            )
        )
    ))
    && (result == SBI_SBI_SUCCESS ==> (
        (forall i: int ::
            (i >= 0 as int) &&
            ((old_s.trig_idx_base as int) + i < (old_s.trig_idx_mask as int)) ==>
            (
                (new_s.trig_mapped(i) == false) &&
                (new_s.trig_state(i) == 0 as int) &&
                (new_s.trig_tdata1(i) == 0 as int) &&
                (new_s.trig_tdata2(i) == 0 as int) &&
                (new_s.trig_tdata3(i) == 0 as int)
            )
        )
    ))
}