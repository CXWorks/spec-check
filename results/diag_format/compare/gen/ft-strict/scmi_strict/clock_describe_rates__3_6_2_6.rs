pub open spec fn clock_describe_rates__3_6_2_6_spec(clock_id: UInt32, rate_index: UInt32, status: Int32, num_rates_flags: UInt32, rates: [UInt64; 4], old_s: S, new_s: S) -> bool {
  (!ClockExists(old_s, clock_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsValidRateIndex(old_s, clock_id, rate_index) ==> ResultEqual(status, OUT_OF_RANGE))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> Bits(num_rates_flags, 15, 13) == 0)
  && (ResultEqual(status, SUCCESS) ==> Bits(num_rates_flags, 12, 12) == 1 ==> Bits(num_rates_flags, 11, 0) == 3)
  && (ResultEqual(status, SUCCESS) ==> Bits(num_rates_flags, 12, 12) == 1 ==> Bits(num_rates_flags, 31, 16) == 0)
  && (ResultEqual(status, SUCCESS) ==> Bits(num_rates_flags, 12, 12) == 1 ==> IsClockRateSegment(old_s, clock_id, LowestRate(rates), HighestRate(rates), StepSize(rates)))
  && (ResultEqual(status, SUCCESS) ==> Bits(num_rates_flags, 12, 12) == 0 ==> (forall i: UInt32| i < Bits(num_rates_flags, 11, 0) ==> IsClockPhysicalRate(old_s, clock_id, RateHz(rates, i as int))))
  && (ResultEqual(status, SUCCESS) ==> Bits(num_rates_flags, 12, 12) == 0 ==> RateHz(rates, 0 as int) == ClockRateAtIndex(old_s, clock_id, rate_index))
  && (ResultEqual(status, SUCCESS) ==> Bits(num_rates_flags, 12, 12) == 0 ==> Bits(num_rates_flags, 31, 16) == NumRemainingRates(old_s, clock_id, rate_index, Bits(num_rates_flags, 11, 0) as int))
  && (ResultEqual(status, SUCCESS) ==> forall i: UInt32| i + 1 < Bits(num_rates_flags, 11, 0) && Bits(num_rates_flags, 12, 12) == 0 ==> RateHz(rates, i as int) < RateHz(rates, (i + 1) as int))
  && (ResultEqual(status, SUCCESS) ==> Bits(num_rates_flags, 11, 0) <= MaxTransportReturnRates())
  && ((ClockExists(old_s, clock_id) &&
       IsValidRateIndex(old_s, clock_id, rate_index))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> Bits(num_rates_flags, 15, 13) == 0)
  && (result != SUCCESS
    ==> Bits(num_rates_flags, 12, 12) == 1 ==> Bits(num_rates_flags, 11, 0) == 3)
  && (result != SUCCESS
    ==> Bits(num_rates_flags, 12, 12) == 1 ==> Bits(num_rates_flags, 31, 16) == 0)
  && (result != SUCCESS
    ==> Bits(num_rates_flags, 12, 12) == 1 ==> IsClockRateSegment(old_s, clock_id, LowestRate(rates), HighestRate(rates), StepSize(rates)))
  && (result != SUCCESS
    ==> Bits(num_rates_flags, 12, 12) == 0 ==> (forall i: UInt32| i < Bits(num_rates_flags, 11, 0) ==> IsClockPhysicalRate(old_s, clock_id, RateHz(rates, i as int))))
  && (result != SUCCESS
    ==> Bits(num_rates_flags, 12, 12) == 0 ==> RateHz(rates, 0 as int) == ClockRateAtIndex(old_s, clock_id, rate_index))
  && (result != SUCCESS
    ==> Bits(num_rates_flags, 12, 12) == 0 ==> Bits(num_rates_flags, 31, 16) == NumRemainingRates(old_s, clock_id, rate_index, Bits(num_rates_flags, 11, 0) as int))
  && (result != SUCCESS
    ==> forall i: UInt32| i + 1 < Bits(num_rates_flags, 11, 0) && Bits(num_rates_flags, 12, 12) == 0 ==> RateHz(rates, i as int) < RateHz(rates, (i + 1) as int))
  && (result != SUCCESS
    ==> Bits(num_rates_flags, 11, 0) <= MaxTransportReturnRates())
}