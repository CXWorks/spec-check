pub open spec fn 3.11.2.11_pinctrl_name_get_spec(identifier: UInt32, flags: UInt32, result: RsiCommandReturnCode, flags_out: UInt32, name: [UInt8; 64], old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> flags_out == 0)
  && (result == RSI_SUCCESS ==> name[0] == 0)
  && ((!(result == RSI_SUCCESS))
    ==> flags_out == 0)
  && ((!(result == RSI_SUCCESS))
    ==> name[0] == 0)
}