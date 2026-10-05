pub open spec fn sdei_event_disable_spec(result: i64, event: i32, old_s: S, new_s: S) -> bool {
    (result == SUCCESS || result == NOT_SUPPORTED || result == INVALID_PARAMETERS || result == DENIED)
    && (result != SUCCESS ==> new_s == old_s)
}
