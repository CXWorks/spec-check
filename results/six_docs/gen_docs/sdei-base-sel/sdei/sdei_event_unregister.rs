pub open spec fn sdei_event_unregister_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SDEI_SUCCESS ==> (new_s.sdei_event_registered(old_s.event) == false))
    && (result == SDEI_NOT_SUPPORTED ==> true)
    && (result == SDEI_INVALID_PARAMETERS ==> true)
    && (result == SDEI_DENIED ==> true)
    && (result == SDEI_PENDING ==> true)
}