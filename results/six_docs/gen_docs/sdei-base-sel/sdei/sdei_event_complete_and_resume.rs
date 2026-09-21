pub open spec fn sdei_event_complete_and_resume_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SDEI_SUCCESS ==> (true))
    && (result == SDEI_NOT_SUPPORTED ==> (true))
    && (result == SDEI_INVALID_PARAMETERS ==> (true))
    && (result == SDEI_DENIED ==> (true))
    && (result == SDEI_OUT_OF_RESOURCE ==> (true))
    && (result == SDEI_PENDING ==> (true))
}