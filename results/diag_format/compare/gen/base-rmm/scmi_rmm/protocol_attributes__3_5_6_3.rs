pub open spec fn protocol_attributes__3_5_6_3_spec(result: int32, attributes: uint32, statistics_address_low: uint32, statistics_address_high: uint32, statistics_len: uint32, old_s: S, new_s: S) -> bool {
    (result == 0)
    && (attributes[31..18] == 0)
    && (attributes[17..16] == 0 || attributes[17..16] == 1 || attributes[17..16] == 2)
    && (attributes[15..0] == NumPerformanceDomains())
    && (statistics_len != 0 ==> (IsInCallerMemoryMap(statistics_address_high, statistics_address_low) && IsAligned64(statistics_address_high, statistics_address_low)))
    && (statistics_len == 0 ==> !PlatformSupportsStatisticsRegion())
    && (old_s == new_s)
}