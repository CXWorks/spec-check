pub open spec fn 3.2.2.13_base_reset_agent_configuration_spec(agent_id: UInt32, flags: UInt32, result: Result<(), RsiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS)
  && (result == RSI_ERROR_NOT_FOUND)
  && (result == RSI_ERROR_INVALID_PARAMETERS)
  && (result == RSI_ERROR_NOT_SUPPORTED)
  && (result == RSI_ERROR_DENIED)
  && ((!(flags & 1) == 0) ==> result == RSI_SUCCESS)
}