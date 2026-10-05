pub open spec fn clock_rate_set_complete__3_6_3_1_spec(status: i32, clock_id: u32, rate_lower: u32, rate_upper: u32, old_s: S, new_s: S) -> bool {
    (status == SUCCESS ==> (
        ClockRateSetSucceeded(old_s, new_s, clock_id)
        && ClockRate(new_s, clock_id) == (rate_upper as int) * 0x1_0000_0000 + (rate_lower as int)
    ))
    && (status == DENIED ==> (
        ClockHasOtherUsers(old_s, clock_id)
        && ClockRate(new_s, clock_id) == ClockRate(old_s, clock_id)
    ))
}
