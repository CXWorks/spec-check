pub open spec fn migrate_info_up_cpu_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_SUCCESS ==> (new_s.migrate_info_up_cpu == old_s.migrate_info_up_cpu))
    && (result == RSI_ERROR_INPUT ==> (new_s.migrate_info_up_cpu == old_s.migrate_info_up_cpu))
    && (result == RSI_ERROR_STATE ==> (new_s.migrate_info_up_cpu == old_s.migrate_info_up_cpu))
    && (result == RSI_INCOMPLETE ==> (new_s.migrate_info_up_cpu == old_s.migrate_info_up_cpu))
    && (result == RSI_ERROR_UNKNOWN ==> (new_s.migrate_info_up_cpu == old_s.migrate_info_up_cpu))
}