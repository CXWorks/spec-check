pub open spec fn sbi_debug_num_triggers_spec(result: SbiReturnCode, value: UInt64, old_s: S, new_s: S, trig_tdata1: UInt64) -> bool {
    (result == SBI_SUCCESS)
    && ((trig_tdata1 == 0) ==> (value as int) == TotalNumDebugTriggers(old_s))
    && ((trig_tdata1 != 0) ==> (value as int) == NumDebugTriggersSupportingConfig(old_s, trig_tdata1))
    && ((trig_tdata1 != 0 && NumDebugTriggersSupportingConfig(old_s, trig_tdata1) == 0) ==> value == 0)
    && (new_s == old_s)
}
