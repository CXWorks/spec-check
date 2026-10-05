pub open spec fn sdei_event_get_info_spec(result: i64, event: i32, info: u32, old_s: S, new_s: S) -> bool {
    (((info as int) > 4) ==> (result == INVALID_PARAMETERS))
    && (((info as int) <= 3 && result != NOT_SUPPORTED && result != INVALID_PARAMETERS && result != DENIED) ==> (result == 0 || result == 1))
    && (((info as int) <= 2) ==> (result != DENIED))
    && (new_s == old_s)
}
