pub open spec fn clock_describe_rates__3_6_2_6_spec(clock_id: UInt32, rate_index: UInt32, status: Int32, num_rates_flags: UInt32, result: Result<(), RsiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  (!ClockExists(old_s, clock_id) ==> ResultEqual(result, NOT_FOUND))
  && (!IsValidRateIndex(old_s, clock_id, rate_index) ==> ResultEqual(result, OUT_OF_RANGE))
  && (result == RSI_SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == RSI_SUCCESS ==> (num_rates_flags & 0x7FF) == (num_rates_flags & 0x7FF))
  && (result == RSI_SUCCESS ==> (num_rates_flags & 0x6000) == 0)
  && (result == RSI_SUCCESS ==> (num_rates_flags & 0x1000) == 1 ==> (num_rates_flags & 0xFFFF0000) == 0)
  && (result == RSI_SUCCESS ==> (num_rates_flags & 0x1000) == 1 ==> (num_rates_flags & 0x7FF) == 3)
  && ((!(ClockExists(old_s, clock_id)) &&
       IsValidRateIndex(old_s, clock_id, rate_index))
    ==> ResultEqual(result, RSI_SUCCESS))
  && (result != RSI_SUCCESS
    ==> (num_rates_flags & 0x7FF) == 0)
  && (result != RSI_SUCCESS
    ==> (num_rates_flags & 0x6000) == 0)
  && (result != RSI_SUCCESS
    ==> (num_rates_flags & 0x1000) == 0)
}