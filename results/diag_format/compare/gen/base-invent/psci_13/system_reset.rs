pub open spec fn system_reset_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_SUCCESS ==> (new_s == old_s))
    && (result != RSI_SUCCESS ==> true)
}