pub open spec fn sbi_debug_uninstall_triggers_spec(trig_idx_base: u64, trig_idx_mask: u64, result: SbiRet, old_s: S, new_s: S) -> bool {
    ((exists|i: int| 0 <= i < 64
        && ((trig_idx_mask >> (i as u64)) & 1u64) == 1u64
        && ((trig_idx_base as int) + i >= DebugTrigMax(old_s)
            || !DebugTrigIsMapped(old_s, (trig_idx_base as int) + i)))
        ==> result.error == SBI_ERR_INVALID_PARAM)
    && ((forall|i: int| 0 <= i < 64
        && ((trig_idx_mask >> (i as u64)) & 1u64) == 1u64
        ==> ((trig_idx_base as int) + i < DebugTrigMax(old_s)
            && DebugTrigIsMapped(old_s, (trig_idx_base as int) + i)))
        ==> (result.error == SBI_SUCCESS
            && (forall|i: int| 0 <= i < 64
                && ((trig_idx_mask >> (i as u64)) & 1u64) == 1u64
                ==> (HwTriggerTdata1(new_s, DebugTrigMappedHwTrigger(old_s, (trig_idx_base as int) + i)) == 0
                    && HwTriggerTdata2(new_s, DebugTrigMappedHwTrigger(old_s, (trig_idx_base as int) + i)) == 0
                    && HwTriggerTdata3(new_s, DebugTrigMappedHwTrigger(old_s, (trig_idx_base as int) + i)) == 0
                    && DebugTrigState(new_s, (trig_idx_base as int) + i) == 0
                    && !DebugTrigIsMapped(new_s, (trig_idx_base as int) + i)
                    && HwTriggerIsFree(new_s, DebugTrigMappedHwTrigger(old_s, (trig_idx_base as int) + i))
                    && DebugTrigIdxIsFree(new_s, (trig_idx_base as int) + i)))
            && (forall|j: int| 0 <= j < DebugTrigMax(old_s)
                && !(j >= (trig_idx_base as int) && j < (trig_idx_base as int) + 64
                    && ((trig_idx_mask >> ((j - (trig_idx_base as int)) as u64)) & 1u64) == 1u64)
                ==> (DebugTrigIsMapped(new_s, j) == DebugTrigIsMapped(old_s, j)
                    && DebugTrigState(new_s, j) == DebugTrigState(old_s, j)
                    && DebugTrigMappedHwTrigger(new_s, j) == DebugTrigMappedHwTrigger(old_s, j)))))
}
