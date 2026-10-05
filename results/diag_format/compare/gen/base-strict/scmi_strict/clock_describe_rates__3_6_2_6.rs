pub open spec fn clock_describe_rates__3_6_2_6_spec(status: Int32, num_rates_flags: UInt32, rates: Array<UInt64>, clock_id: UInt32, rate_index: UInt32, old_s: S, new_s: S) -> bool {
    (!ClockExists(clock_id) ==> ResultEqual(status, NOT_FOUND))
    && (!IsValidRateIndex(clock_id, rate_index) ==> ResultEqual(status, OUT_OF_RANGE))
    && (ResultEqual(status, SUCCESS) ==> (
        Bits64(num_rates_flags, 15, 13) == 0
        && (Bits64(num_rates_flags, 12, 12) == 1 ==> (
            Bits64(num_rates_flags, 11, 0) == 3
            && Bits64(num_rates_flags, 31, 16) == 0
            && IsClockRateSegment(clock_id, LowestRate(rates), HighestRate(rates), StepSize(rates))
        ))
        && (Bits64(num_rates_flags, 12, 12) == 0 ==> (
            forall|i: UInt32| i < Bits64(num_rates_flags, 11, 0) ==> IsClockPhysicalRate(clock_id, RateHz(rates, i))
            && RateHz(rates, 0) == ClockRateAtIndex(clock_id, rate_index)
            && Bits64(num_rates_flags, 31, 16) == NumRemainingRates(clock_id, rate_index, Bits64(num_rates_flags, 11, 0))
            && (forall|i: UInt32| i + 1 < Bits64(num_rates_flags, 11, 0) ==> RateHz(rates, i) < RateHz(rates, i + 1))
            && Bits64(num_rates_flags, 11, 0) <= MaxTransportReturnRates()
        ))
    ))
    && (old_s == new_s)
}