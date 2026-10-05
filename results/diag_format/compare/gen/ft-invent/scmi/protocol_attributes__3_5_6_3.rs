pub open spec fn protocol_attributes__3_5_6_3_spec(attributes: UInt32, statistics_address_low: UInt32, statistics_address_high: UInt32, statistics_len: UInt32, result: Result<(), RsiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  result == RSI_SUCCESS
}