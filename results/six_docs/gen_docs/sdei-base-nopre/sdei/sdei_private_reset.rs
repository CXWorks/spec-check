pub open spec fn sdei_private_reset_spec(result: i64, old_s: S, new_s: S) -> bool {
    (result == 0 ==> (new_s.sdei_private_event_count == 0))
    && (result == -1 ==> (old_s.sdei_private_event_count > 0))
    && (result == -2 ==> (old_s.sdei_private_event_count > 0))
    && (result == -3 ==> (old_s.sdei_private_event_count > 0))
    && (result != 0 && result != -1 && result != -2 && result != -3 ==> true)
}