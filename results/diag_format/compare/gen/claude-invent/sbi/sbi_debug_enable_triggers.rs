pub open spec fn sbi_debug_enable_triggers_spec(result: SbiErrorCode, trig_idx_base: UInt64, trig_idx_mask: UInt64, old_s: S, new_s: S) -> bool {
    ((exists|i: int| 0 <= i < 64
        && ((trig_idx_mask >> (i as u64)) & 1u64) == 1u64
        && ((trig_idx_base as int) + i >= TrigMax(old_s) as int
            || !TriggerIsMapped(old_s, (trig_idx_base as int) + i)))
        ==> (result == SBI_ERR_INVALID_PARAM && new_s == old_s))
    && ((forall|i: int| 0 <= i < 64
        && ((trig_idx_mask >> (i as u64)) & 1u64) == 1u64
        ==> ((trig_idx_base as int) + i < TrigMax(old_s) as int
            && TriggerIsMapped(old_s, (trig_idx_base as int) + i)))
        ==> (result == SBI_SUCCESS
            && (forall|i: int| 0 <= i < 64
                && ((trig_idx_mask >> (i as u64)) & 1u64) == 1u64
                ==> (MappedHwTrigger(new_s, (trig_idx_base as int) + i).vs == TrigState(old_s, (trig_idx_base as int) + i).vs
                    && MappedHwTrigger(new_s, (trig_idx_base as int) + i).vu == TrigState(old_s, (trig_idx_base as int) + i).vu
                    && MappedHwTrigger(new_s, (trig_idx_base as int) + i).s == TrigState(old_s, (trig_idx_base as int) + i).s
                    && MappedHwTrigger(new_s, (trig_idx_base as int) + i).u == TrigState(old_s, (trig_idx_base as int) + i).u))))
}
