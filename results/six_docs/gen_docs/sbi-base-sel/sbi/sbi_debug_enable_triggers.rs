pub open spec fn sbi_debug_enable_triggers_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_INVALID_PARAM ==> (
        (exists (trig_idx: UInt64).
            (trig_idx >= old_s.trig_max)
            || (exists (trig_idx: UInt64).
                (trig_idx >= old_s.trig_idx_base)
                && (trig_idx < old_s.trig_idx_base + old_s.trig_idx_mask)
                && (exists (hw_trig: UInt64).
                    (hw_trig < old_s.hw_trig_count)
                    && (old_s.trig_state[trig_idx] != old_s.hw_trig_state[hw_trig]))
                ))
        )
    )
    && (result == SBI_SBI_SUCCESS ==> (
        (forall (trig_idx: UInt64).
            (trig_idx >= old_s.trig_idx_base)
            && (trig_idx < old_s.trig_idx_base + old_s.trig_idx_mask)
            ==> (exists (hw_trig: UInt64).
                (hw_trig < old_s.hw_trig_count)
                && (old_s.trig_state[trig_idx] == old_s.hw_trig_state[hw_trig])
                && (new_s.trig_state[trig_idx] == old_s.trig_state[trig_idx])
                && (new_s.hw_trig_state[hw_trig] == old_s.trig_state[trig_idx]))
            ))
    ))
}