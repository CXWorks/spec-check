pub open spec fn clock_rate_get__3_6_2_8_spec(result: Int32, rate_low: UInt32, rate_high: UInt32, old_s: S, new_s: S) -> bool {
    (!ClockExists(old_s, clock_id) ==> ResultEqual(result, NOT_FOUND))
    && (ResultEqual(result, SUCCESS) ==> (
        (!ClockIsDisabled(old_s, clock_id) ==> (rate_low + rate_high * 4294967296 == ClockCurrentRate(old_s, clock_id)))
        && (ClockIsDisabled(old_s, clock_id) ==> (rate_low + rate_high * 4294967296 == ClockRateOnReenable(old_s, clock_id)))
    ))
    && (old_s == new_s)
}