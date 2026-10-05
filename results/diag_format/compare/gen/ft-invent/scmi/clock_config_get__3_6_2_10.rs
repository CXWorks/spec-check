pub open spec fn clock_config_get__3_6_2_10_spec(clock_id: UInt32, flags: UInt32, result: RsiCommandReturnCode, attributes: UInt32, config: UInt32, extended_config_val: UInt32, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> (attributes & 0xFF) == 0)
  && (result == RSI_SUCCESS ==> (config & 0xFFFFFFFE) == 0)
  && ((!(result == RSI_SUCCESS))
    ==> (attributes == 0))
  && ((!(result == RSI_SUCCESS))
    ==> (config == 0))
  && ((!(result == RSI_SUCCESS))
    ==> (extended_config_val == 0))
}