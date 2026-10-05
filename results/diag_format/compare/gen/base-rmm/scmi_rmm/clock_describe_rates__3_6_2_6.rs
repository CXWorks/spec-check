pub open spec fn clock_describe_rates__3_6_2_6_spec(status: Int32, num_rates_flags: UInt32, rates: {UInt32, UInt32}, old_s: S, new_s: S) -> bool {
    (!ClockExists(old_s, clock_id) ==> ResultEqual(status, NOT_FOUND))
    && (!IsValidRateIndex(old_s, clock_id, rate_index) ==> ResultEqual(status, OUT_OF_RANGE))
    && (ResultEqual(status, SUCCESS) ==> (N == num_rates_flags[11:0]))
    && (ResultEqual(status, SUCCESS) ==> (num_rates_flags[15:13] == 0))
    && ((num_rates_flags[12] == 1) ==> (num_rates_flags[31:16] == 0))
    && ((num_rates_flags[12] == 1) ==> (num_rates_flags[11:0] == 3))
    && (ResultEqual(status, SUCCESS) ==> (rates[0..N-1] is_numeric_ascending))
    && (old_s == new_s)
}