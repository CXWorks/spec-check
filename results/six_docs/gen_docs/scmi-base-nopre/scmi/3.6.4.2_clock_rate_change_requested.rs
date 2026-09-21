pub open spec fn 3.6.4.2_clock_rate_change_requested_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_SUCCESS ==> (new_s == old_s))
}