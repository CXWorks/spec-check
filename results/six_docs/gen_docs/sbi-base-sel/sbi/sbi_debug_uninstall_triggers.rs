pub open spec fn sbi_debug_uninstall_triggers_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_INVALID_PARAM ==> (
        (forall i: int, i >= 0 && i < 64 =>
            let trig_idx: int = (old_s.trig_idx_base as int) + (i as int);
            let mask_bit: int = (old_s.trig_idx_mask as int) >> i & 1;
            mask_bit == 1 ==> (
                (trig_idx >= (old_s.trig_max as int)) ||
                (forall j: int, j >= 0 && j < 64 =>
                    let trig_idx_j: int = (old_s.trig_idx_base as int) + (j as int);
                    let mask_bit_j: int = (old_s.trig_idx_mask as int) >> j & 1;
                    mask_bit_j == 1 ==> (
                        (trig_idx_j as int) != trig_idx ||
                        (old_s.trig_mapped(trig_idx_j) == false)
                    )
                )
            )
        )
    ))
    && (result == SBI_SBI_SUCCESS ==> (
        (forall i: int, i >= 0 && i < 64 =>
            let trig_idx: int = (old_s.trig_idx_base as int) + (i as int);
            let mask_bit: int = (old_s.trig_idx_mask as int) >> i & 1;
            mask_bit == 1 ==> (
                trig_idx >= 0 && trig_idx < (old_s.trig_max as int) &&
                old_s.trig_mapped(trig_idx) &&
                new_s.trig_mapped(trig_idx) == false &&
                new_s.trig_state(trig_idx) == 0 &&
                new_s.trig_tdata1(trig_idx) == 0 &&
                new_s.trig_tdata2(trig_idx) == 0 &&
                new_s.trig_tdata3(trig_idx) == 0
            )
        )
    ))
}