pub open spec fn sbi_debug_read_triggers_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_SUCCESS ==>
        forall|i: UInt64| (i < trig_count(old_s) ==> SharedMemoryHoldsTriggerStateLE(CallingHart(), trig_idx_base(old_s) + i, i * (XLEN / 2), 4)))
}