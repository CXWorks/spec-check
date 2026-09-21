pub open spec fn sdei_event_status_spec(result: int64, old_s: S, new_s: S) -> bool {
    (result == SDEI_ERROR_NOT_SUPPORTED ==> true)
    && (result == SDEI_ERROR_INVALID_PARAMETERS ==> true)
    && (result != SDEI_ERROR_NOT_SUPPORTED && result != SDEI_ERROR_INVALID_PARAMETERS ==> (result as int64) >= 0)
}