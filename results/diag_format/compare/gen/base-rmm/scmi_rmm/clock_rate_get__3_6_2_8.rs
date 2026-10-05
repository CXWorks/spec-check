pub open spec fn clock_rate_get__3_6_2_8_spec(result: Int32, rate: [UInt32; 2], old_s: S, new_s: S) -> bool {
    (!ClockExists(old_s, clock_id) ==> ResultEqual(result, NOT_FOUND))
    && (ClockIsEnabled(old_s, clock_id) ==> ResultEqual(result, SUCCESS) && ((rate[1] as u64) << 32 | rate[0]) == ClockCurrentRate(old_s, clock_id))
    && (!ClockIsEnabled(old_s, clock_id) && ResultEqual(result, SUCCESS) ==> ((rate[1] as u64) << 32 | rate[0]) == ClockRateOnReenable(old_s, clock_id))
    && (old_s == new_s)
}