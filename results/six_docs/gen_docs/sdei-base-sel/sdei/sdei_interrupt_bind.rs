pub open spec fn sdei_interrupt_bind_spec(result: int64, old_s: S, new_s: S) -> bool {
    (result == SDEI_SUCCESS ==> (
        (old_s.sdei_event_count as int) < (new_s.sdei_event_count as int)
        && (new_s.sdei_event_count as int) <= (old_s.sdei_event_count as int) + 1
        && (new_s.sdei_event_count as int) <= SDEI_MAX_EVENTS
        && (result as int) >= 0
        && (result as int) < (new_s.sdei_event_count as int)
    ))
    && (result == SDEI_DENIED ==> (
        !old_s.interrupt_is_inactive(old_s.interrupt)
    ))
    && (result == SDEI_INVALID_PARAMETERS ==> (
        !old_s.interrupt_is_owned(old_s.interrupt)
        || !old_s.interrupt_is_valid(old_s.interrupt)
    ))
    && (result == SDEI_OUT_OF_RESOURCE ==> (
        (old_s.sdei_event_count as int) >= SDEI_MAX_EVENTS
    ))
    && (result == SDEI_NOT_SUPPORTED ==> (
        !old_s.sdei_is_supported()
    ))
    && (result != SDEI_SUCCESS && result != SDEI_DENIED && result != SDEI_INVALID_PARAMETERS && result != SDEI_OUT_OF_RESOURCE && result != SDEI_NOT_SUPPORTED ==> true)
}