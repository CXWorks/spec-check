pub open spec fn sbi_debug_update_triggers_spec(result: int, trig_count: UInt64, old_s: S, new_s: S) -> bool {
    (trig_count == 0 ==> result == SBI_SBI_SUCCESS)
    && (trig_count > 0 ==> result == SBI_SBI_SUCCESS)
}