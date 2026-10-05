pub open spec fn sbi_debug_disable_triggers_spec(trig_idx_base: UInt64, trig_idx_mask: UInt64, result: SbiRet, old_s: S, new_s: S) -> bool {
    (
        (exists|i: int| 0 <= i < 64
            && ((trig_idx_mask >> (i as u64)) & 1u64) == 1u64
            && (
                !DebugTriggerIsMapped(old_s, (trig_idx_base as int) + i)
                || (trig_idx_base as int) + i >= DebugTriggerMax(old_s) as int
            ))
        ==> result.error == SBI_ERR_INVALID_PARAM
    )
    && (
        (forall|i: int| 0 <= i < 64
            && ((trig_idx_mask >> (i as u64)) & 1u64) == 1u64
            ==> (
                DebugTriggerIsMapped(old_s, (trig_idx_base as int) + i)
                && (trig_idx_base as int) + i < DebugTriggerMax(old_s) as int
            ))
        ==> (
            result.error == SBI_SUCCESS
            && (forall|i: int| 0 <= i < 64
                && ((trig_idx_mask >> (i as u64)) & 1u64) == 1u64
                ==> (
                    HwTriggerVsBit(new_s, (trig_idx_base as int) + i) == 0
                    && HwTriggerVuBit(new_s, (trig_idx_base as int) + i) == 0
                    && HwTriggerSBit(new_s, (trig_idx_base as int) + i) == 0
                    && HwTriggerUBit(new_s, (trig_idx_base as int) + i) == 0
                ))
            && (forall|idx: int|
                !(exists|i: int| 0 <= i < 64
                    && ((trig_idx_mask >> (i as u64)) & 1u64) == 1u64
                    && idx == (trig_idx_base as int) + i)
                ==> DebugTriggerStateEqual(old_s, new_s, idx))
        )
    )
}
