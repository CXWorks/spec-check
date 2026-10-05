pub open spec fn clock_describe_rates__3_6_2_6_spec(clock_id: u32, rate_index: u32, status: i32, num_rates_flags: u32, rates: Seq<u64>, old_s: S, new_s: S) -> bool {
    (!ClockExists(old_s, clock_id) ==> status == NOT_FOUND)
    && ((ClockExists(old_s, clock_id) && !ClockRateIndexInRange(old_s, clock_id, rate_index)) ==> status == OUT_OF_RANGE)
    && ((ClockExists(old_s, clock_id) && ClockRateIndexInRange(old_s, clock_id, rate_index)) ==> (
        status == SUCCESS
        && ((num_rates_flags >> 13u32) & 0x7u32) == 0u32
        && (rates.len() as int) == ((num_rates_flags & 0xFFFu32) as int)
        && (((num_rates_flags >> 12u32) & 1u32) == 1u32 ==> (
            ((num_rates_flags >> 16u32) & 0xFFFFu32) == 0u32
            && (num_rates_flags & 0xFFFu32) == 3u32
        ))
        && (((num_rates_flags >> 12u32) & 1u32) == 0u32 ==> (
            forall|i: int, j: int| 0 <= i < j < rates.len() ==> rates[i] < rates[j]
        ))
    ))
    && new_s == old_s
}
