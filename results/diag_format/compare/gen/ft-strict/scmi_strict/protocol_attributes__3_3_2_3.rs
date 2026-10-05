pub open spec fn protocol_attributes__3_3_2_3_spec(attributes: UInt32, statistics_address_low: UInt32, statistics_address_high: UInt32, statistics_len: UInt32, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (Bits(attributes, 31, 16) == 0)
  && (Bits(attributes, 15, 0) == NumPowerDomains())
  && ((statistics_len == 0) == !PlatformSupportsStatisticsRegion())
  && (statistics_len != 0 ==> statistics_len == StatisticsRegionLength())
  && (statistics_len != 0 ==> statistics_address_high * 4294967296 + statistics_address_low == StatisticsRegionAddress())
  && (statistics_len != 0 ==> IsInCallerMemoryMap(statistics_address_high * 4294967296 + statistics_address_low))
  && ((!(Bits(attributes, 31, 16) == 0) &&
       !(Bits(attributes, 15, 0) == NumPowerDomains()) &&
       !((statistics_len == 0) == !PlatformSupportsStatisticsRegion()) &&
       !(statistics_len != 0 ==> statistics_len == StatisticsRegionLength()) &&
       !(statistics_len != 0 ==> statistics_address_high * 4294967296 + statistics_address_low == StatisticsRegionAddress()) &&
       !(statistics_len != 0 ==> IsInCallerMemoryMap(statistics_address_high * 4294967296 + statistics_address_low)))
    ==> result == RSI_ERROR_INPUT)
}