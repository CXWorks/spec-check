pub open spec fn system_off_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_SUCCESS ==> true)
    && (result == RSI_ERROR_INPUT ==> true)
    && (result == RSI_ERROR_STATE ==> true)
    && (result == RSI_INCOMPLETE ==> true)
    && (result == RSI_ERROR_UNKNOWN ==> true)
}