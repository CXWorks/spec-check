pub open spec fn ffa_rxtx_map_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_ERROR_INPUT ==> <failure conditions for invalid inputs>)
    && (result == RSI_ERROR_STATE ==> <failure conditions for invalid state>)
    && (result == RSI_INCOMPLETE ==> <failure conditions for incomplete state>)
    && (result == RSI_ERROR_UNKNOWN ==> <failure conditions for unknown commands>)
    && (result == RSI_SUCCESS ==> <success conditions for valid mapping>)
}