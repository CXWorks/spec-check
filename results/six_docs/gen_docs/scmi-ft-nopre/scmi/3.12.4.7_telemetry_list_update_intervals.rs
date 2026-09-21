pub open spec fn 3.12.4.7_telemetry_list_update_intervals_spec(index: UInt32, group_identifier: UInt32, flags: UInt32, result: Result<(), RsiCommandReturnCode>, flags_out: UInt32, intervals: [UInt32; 4], old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> flags_out == 0)
  && (result == RSI_SUCCESS ==> (flags_out & 0x1_000) == 0)
  && ((!(result == RSI_SUCCESS) &&
       !(result == RSI_ERROR_INPUT) &&
       !(result == RSI_ERROR_STATE) &&
       !(result == RSI_INCOMPLETE) &&
       !(result == RSI_ERROR_UNKNOWN))
    ==> flags_out == flags)
}