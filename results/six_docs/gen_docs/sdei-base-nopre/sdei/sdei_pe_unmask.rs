pub open spec fn sdei_pe_unmask_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_ERROR_INPUT ==> <unconstrained>)
    && (result == RSI_SUCCESS ==> <unconstrained>)
    && (result == RSI_ERROR_STATE ==> <unconstrained>)
    && (result == RSI_INCOMPLETE ==> <unconstrained>)
    && (result == RSI_ERROR_UNKNOWN ==> <unconstrained>)
}