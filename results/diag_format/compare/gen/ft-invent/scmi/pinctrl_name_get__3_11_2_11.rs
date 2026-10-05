pub open spec fn pinctrl_name_get__3_11_2_11_spec(identifier: UInt32, flags: UInt32, result: RsiCommandReturnCode, flags_out: UInt32, name: [UInt8; 64], old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> flags_out == 0)
  && ((!(result == RSI_SUCCESS))
    ==> flags_out == 0)
}