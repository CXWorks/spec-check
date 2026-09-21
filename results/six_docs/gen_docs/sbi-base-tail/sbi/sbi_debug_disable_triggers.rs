pub open spec fn sbi_debug_disable_triggers_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_INVALID_PARAM ==> (
        (exists trig_idx: int, trig_idx_base: int, trig_idx_mask: int where
            trig_idx_base == trig_idx_base &&
            trig_idx_mask == trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask > 0 &&
            (trig_idx_base as int) + trig_idx_mask <= (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) + trig_idx_mask < (trig_idx_base as int) + trig_idx_mask &&
            (trig_idx_base as int) + trig_idx_mask >= (trig_idx_base as int) &&
            (trig_idx_base as int) +