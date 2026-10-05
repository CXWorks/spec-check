pub open spec fn protocol_attributes__3_10_3_3_spec(attributes: uint32, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> Bits(attributes, 31, 16) == 0)
  && (result == RSI_SUCCESS ==> Bits(attributes, 15, 0) == NumPowerCappingDomains())
  && ((!(result == RSI_SUCCESS)) ==> true)
}