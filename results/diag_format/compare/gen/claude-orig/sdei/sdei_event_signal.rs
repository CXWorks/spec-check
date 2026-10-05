pub open spec fn sdei_event_signal_spec(event: i32, target_pe: UInt64, result: i64, old_s: S, new_s: S) -> bool {
    (event != 0 ==> result == INVALID_PARAMETERS)
    && (result == SUCCESS || result == NOT_SUPPORTED || result == INVALID_PARAMETERS)
}
