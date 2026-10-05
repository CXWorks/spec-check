pub open spec fn protocol_attributes__3_3_2_3_spec(result: Result<(), RmiStatusCode>, status: Int32, attributes: UInt32, statistics_address_low: UInt32, statistics_address_high: UInt32, statistics_len: UInt32, old_s: S, new_s: S) -> bool {
    (result.is_Ok() && status == SUCCESS)
    && (attributes[31..16] == 0)
    && (attributes[15..0] == NumPowerDomains())
    && ((statistics_len == 0) == !PlatformSupportsStatisticsRegion())
    && (statistics_len != 0 ==> StatisticsRegionAddr(statistics_address_high, statistics_address_low) == StatisticsRegionBase())
    && (statistics_len != 0 ==> IsInCallerMemoryMap(StatisticsRegionAddr(statistics_address_high, statistics_address_low)))
    && (statistics_len != 0 ==> statistics_len == StatisticsRegionSize())
    && (old_s == new_s)
}