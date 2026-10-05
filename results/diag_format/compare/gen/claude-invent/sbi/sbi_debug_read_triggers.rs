pub open spec fn sbi_debug_read_triggers_spec(result: SbiRet, trig_idx_base: u64, trig_count: u64, old_s: S, new_s: S) -> bool {
    (result.error == SBI_SUCCESS ==>
        (forall|i: int| 0 <= i < (trig_count as int) ==>
            ShmemDebugTriggerEntryEqual(
                new_s,
                i * (Xlen(old_s) as int / 2),
                DebugTriggerStateAndConfig(old_s, CallingHart(old_s), (trig_idx_base as int) + i)
            ))
        && DebugTriggersUnchanged(old_s, new_s, CallingHart(old_s)))
}
