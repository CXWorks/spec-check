pub open spec fn sdei_event_signal_spec(event: int32, target_pe: UInt64, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS && event != 0 ==> RSI_ERROR_INVALID_PARAMETERS)
  && (result == RSI_SUCCESS && target_pe == 0 ==> RSI_ERROR_INVALID_PARAMETERS)
  && ((!(result == RSI_SUCCESS && event != 0) &&
       !(result == RSI_SUCCESS && target_pe == 0))
    ==> result == RSI_SUCCESS)
}