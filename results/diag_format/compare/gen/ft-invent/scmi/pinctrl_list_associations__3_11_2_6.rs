pub open spec fn pinctrl_list_associations__3_11_2_6_spec(identifier: UInt32, flags: UInt32, index: UInt32, result: RsiCommandReturnCode, flags_out: UInt32, array: [UInt16; 4], old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> flags_out >= 0)
  && (result == RSI_SUCCESS ==> array.len >= 0)
  && ((!(flags & 3) == 0) ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS ==> flags_out == 0)
  && (result != RSI_SUCCESS ==> array[0] == 0)
  && (result != RSI_SUCCESS ==> array[1] == 0)
  && (result != RSI_SUCCESS ==> array[2] == 0)
  && (result != RSI_SUCCESS ==> array[3] == 0)
}