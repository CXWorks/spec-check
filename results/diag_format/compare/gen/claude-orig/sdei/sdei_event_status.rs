pub open spec fn sdei_event_status_spec(event: i32, result: i64, old_s: S, new_s: S) -> bool {
    ((result >= 0) || (result == -1i64) || (result == -2i64))
    && ((result >= 0) ==> (result < 8))
    && (new_s == old_s)
}
