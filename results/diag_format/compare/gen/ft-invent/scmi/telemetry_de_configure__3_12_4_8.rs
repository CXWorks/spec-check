pub open spec fn telemetry_de_configure__3_12_4_8_spec(identifier: UInt32, flags: UInt32, result: RsiCommandReturnCode, shmti_id: UInt32, shmti_de_offset: UInt32, blk_ts_offset: UInt32, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> shmti_id == 0xFFFFFFFF)
  && (result == RSI_SUCCESS && (flags & 3) == 0 ==> shmti_de_offset == 0)
  && (result == RSI_SUCCESS && (flags & 3) == 0 ==> blk_ts_offset == 0)
  && ((!(result == RSI_SUCCESS) &&
       !(result == RSI_ERROR_INPUT) &&
       !(result == RSI_ERROR_STATE) &&
       !(result == RSI_INCOMPLETE) &&
       !(result == RSI_ERROR_UNKNOWN))
    ==> shmti_id == 0)
  && (result == RSI_ERROR_INPUT
    ==> shmti_id == 0)
  && (result == RSI_ERROR_STATE
    ==> shmti_id == 0)
  && (result == RSI_INCOMPLETE
    ==> shmti_id == 0)
  && (result == RSI_ERROR_UNKNOWN
    ==> shmti_id == 0)
}