pub open spec fn protocol_attributes__3_5_6_3_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_SUCCESS)
    && (Bits(new_s.attributes, 31, 18) == 0)
    && (Bits(new_s.attributes, 17, 16) <= 2)
    && ((Bits(new_s.attributes, 17, 16) == 2) ==> PowerExpressedInMicrowatts())
    && ((Bits(new_s.attributes, 17, 16) == 1) ==> PowerExpressedInMilliwatts())
    && ((Bits(new_s.attributes, 17, 16) == 0) ==> PowerExpressedInAbstractLinearScale())
    && (Bits(new_s.attributes, 15, 0) == NumPerformanceDomains())
    && ((new_s.statistics_len == 0) ==> !StatisticsRegionSupported())
    && ((new_s.statistics_len != 0) ==> (new_s.statistics_address_low % 8 == 0))
    && ((new_s.statistics_len != 0) ==> AddrInCallerMemoryMap((new_s.statistics_address_high as u64) * 4294967296 + new_s.statistics_address_low))
}