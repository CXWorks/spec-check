pub open spec fn sdei_pe_unmask_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_ERROR_INPUT ==> <failure conditions>)
    && (result == RSI_ERROR_STATE ==> <failure conditions>)
    && (result == RSI_INCOMPLETE ==> <failure conditions>)
    && (result == RSI_ERROR_UNKNOWN ==> <failure conditions>)
    && (result == RSI_SUCCESS ==> <success conditions>)
}