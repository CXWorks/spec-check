pub open spec fn sdei_event_register_spec(result: int64, old_s: S, new_s: S) -> bool {
    (result == SDEI_INVALID_PARAMETERS ==> (old_s.event_count < 256 || (old_s.event_count > 0 && !old_s.event_registered(old_s.event_count - 1))))
    && (result == SDEI_DENIED ==> old_s.event_registered(old_s.event_count - 1))
    && (result == SDEI_NOT_SUPPORTED ==> true)
    && (result == SDEI_OUT_OF_RESOURCE ==> true)
    && (result == SDEI_SUCCESS ==> (new_s.event_count == old_s.event_count + 1) && new_s.event_registered(old_s.event_count - 1))
    && (result == SDEI_PENDING ==> true)
}