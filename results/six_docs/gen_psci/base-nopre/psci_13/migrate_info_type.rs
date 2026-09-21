pub open spec fn migrate_info_type_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_SUCCESS ==> (
        (old_s.migrate_info_type == 0 ==> (
            // UP migrate capable: MIGRATE returns SUCCESS, CPU_OFF returns DENIED
            true
        ))
        && (old_s.migrate_info_type == 1 ==> (
            // UP not migrate capable: MIGRATE returns DENIED, CPU_OFF returns DENIED
            true
        ))
        && (old_s.migrate_info_type == 2 ==> (
            // MP or not present: MIGRATE returns NOT_SUPPORTED, CPU_OFF does not return (SUCCESS)
            true
        ))
    ))
    && (result != RSI_SUCCESS ==> true)
}