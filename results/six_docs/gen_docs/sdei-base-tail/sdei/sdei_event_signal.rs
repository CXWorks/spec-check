pub open spec fn sdei_event_signal_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SDEI_SUCCESS ==> true)
    && (result == SDEI_NOT_SUPPORTED ==> true)
    && (result == SDEI_INVALID_PARAMETERS ==> true)
}