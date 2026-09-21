pub open spec fn 3.6.2.11_clock_name_get_spec(clock_id: UInt32, result: RsiCommandReturnCode, flags: UInt32, name: [UInt8; 64], old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> flags == 0)
  && (result != RSI_SUCCESS ==> flags == 0)
  && (result == RSI_SUCCESS ==> name[0] == 0)
}