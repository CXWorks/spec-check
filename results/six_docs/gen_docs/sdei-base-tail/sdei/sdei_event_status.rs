pub open spec fn sdei_event_status_spec(result: int64, old_s: S, new_s: S) -> bool {
    (result == SDEI_SUCCESS ==> (
        (result as int64) & 0x7 == 0
        && (result as int64) & 0x4 == 0 || (result as int64) & 0x4 == 1
        && (result as int64) & 0x2 == 0 || (result as int64) & 0x2 == 1
        && (result as int64) & 0x1 == 0 || (result as int64) & 0x1 == 1
    ))
    && (result == SDEI_NOT_SUPPORTED ==> true)
    && (result == SDEI_INVALID_PARAMETERS ==> true)
}