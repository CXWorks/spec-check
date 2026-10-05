pub open spec fn sbi_debug_num_triggers_spec(result: Result<(), RsiCommandReturnCode>, old_s: S, new_s: S) -> bool {
    (result == RSI_SUCCESS)
    && (result.value() == NumTriggers(trig_tdata1(old_s)))
}