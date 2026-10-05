pub open spec fn sbi_debug_install_triggers_spec(trig_count: u64, error: i64, value: u64, old_s: S, new_s: S) -> bool {
    (error == 0 ==> (
        (forall|i: int| 0 <= i < trig_count as int ==> {
            let cfg_off = i * (Xlen() / 2);
            let word_bytes = Xlen() / 8;
            let tdata1 = ShmemWordLe(old_s, cfg_off + word_bytes);
            let tdata2 = ShmemWordLe(old_s, cfg_off + 2 * word_bytes);
            let tdata3 = ShmemWordLe(old_s, cfg_off + 3 * word_bytes);
            let trig_idx = ShmemWordLe(new_s, cfg_off);
            let hw = TrigIdxHwTrigger(new_s, trig_idx);
            TrigIdxIsUnused(old_s, trig_idx)
            && HwTriggerIsUnused(old_s, hw)
            && HwTriggerMatchesConfig(old_s, hw, tdata1, tdata2, tdata3)
            && TrigStateSavedModeBits(new_s, trig_idx) == Tdata1ModeBits(tdata1)
            && HwTriggerTdata1(new_s, hw) == tdata1
            && HwTriggerTdata2(new_s, hw) == tdata2
            && HwTriggerTdata3(new_s, hw) == tdata3
        })
        && (forall|i: int, j: int| 0 <= i < trig_count as int && 0 <= j < trig_count as int && i != j ==>
            ShmemWordLe(new_s, i * (Xlen() / 2)) != ShmemWordLe(new_s, j * (Xlen() / 2))
            && TrigIdxHwTrigger(new_s, ShmemWordLe(new_s, i * (Xlen() / 2)))
                != TrigIdxHwTrigger(new_s, ShmemWordLe(new_s, j * (Xlen() / 2))))
        && (forall|i: int| 0 <= i && i + 1 < trig_count as int
            && TrigConfigChainedToNext(old_s, i) ==>
            (ShmemWordLe(new_s, (i + 1) * (Xlen() / 2)) as int) == (ShmemWordLe(new_s, i * (Xlen() / 2)) as int) + 1
            && (TrigIdxHwTrigger(new_s, ShmemWordLe(new_s, (i + 1) * (Xlen() / 2))) as int)
                == (TrigIdxHwTrigger(new_s, ShmemWordLe(new_s, i * (Xlen() / 2))) as int) + 1)
    ))
}
