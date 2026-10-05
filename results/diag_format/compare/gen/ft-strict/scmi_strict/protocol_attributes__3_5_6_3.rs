pub open spec fn protocol_attributes__3_5_6_3_spec(attributes: UInt32, statistics_address_low: UInt32, statistics_address_high: UInt32, statistics_len: UInt32, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (Bits(attributes, 31, 18) == 0)
  && (Bits(attributes, 17, 16) <= 2)
  && ((Bits(attributes, 17, 16) == 2) ==> PowerExpressedInMicrowatts())
  && ((Bits(attributes, 17, 16) == 1) ==> PowerExpressedInMilliwatts())
  && ((Bits(attributes, 17, 16) == 0) ==> PowerExpressedInAbstractLinearScale())
  && (Bits(attributes, 15, 0) == NumPerformanceDomains())
  && ((statistics_len == 0) ==> !StatisticsRegionSupported())
  && ((statistics_len != 0) ==> (statistics_address_low % 8 == 0))
  && ((statistics_len != 0) ==> AddrInCallerMemoryMap(statistics_address_high * 4294967296 + statistics_address_low))
  && ((!(Bits(attributes, 31, 18) == 0) ||
       !(Bits(attributes, 17, 16) <= 2) ||
       !((Bits(attributes, 17, 16) == 2) ==> PowerExpressedInMicrowatts()) ||
       !((Bits(attributes, 17, 16) == 1) ==> PowerExpressedInMilliwatts()) ||
       !((Bits(attributes, 17, 16) == 0) ==> PowerExpressedInAbstractLinearScale()) ||
       !(Bits(attributes, 15, 0) == NumPerformanceDomains()) ||
       !((statistics_len == 0) ==> !StatisticsRegionSupported()) ||
       !((statistics_len != 0) ==> (statistics_address_low % 8 == 0)) ||
       !((statistics_len != 0) ==> AddrInCallerMemoryMap(statistics_address_high * 4294967296 + statistics_address_low)))
    ==> result != RSI_SUCCESS)
}