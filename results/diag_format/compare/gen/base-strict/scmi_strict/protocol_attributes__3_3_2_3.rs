pub open spec fn protocol_attributes__3_3_2_3_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (true ==> result == RSI_SUCCESS)
    && (true ==> Bits(new_s.attributes, 31, 16) == 0)
    && (true ==> Bits(new_s.attributes, 15, 0) == NumPowerDomains())
    && ((new_s.statistics_len == 0) == !PlatformSupportsStatisticsRegion())
    && (new_s.statistics_len != 0 ==> new_s.statistics_len == StatisticsRegionLength())
    && (new_s.statistics_len != 0 ==> (new_s.statistics_address_high as u64) * 4294967296 + (new_s.statistics_address_low as u64) == StatisticsRegionAddress())
    && (new_s.statistics_len != 0 ==> IsInCallerMemoryMap((new_s.statistics_address_high as u64) * 4294967296 + (new_s.statistics_address_low as u64)))
}