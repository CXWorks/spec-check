pub open spec fn cpu_on_spec(entry_point: Address, context_id: ContextId, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == RSI_ERROR_INVALID_PARAMETERS ==> true)
  && (result == RSI_ERROR_INVALID_ADDRESS ==> true)
  && (result == RSI_ERROR_ALREADY_ON ==> true)
  && (result == RSI_ERROR_ON_PENDING ==> true)
  && (result == RSI_ERROR_INTERNAL_FAILURE ==> true)
  && (result == RSI_ERROR_DENIED ==> true)
  && ((!(result == RSI_ERROR_INVALID_PARAMETERS) &&
       result == RSI_SUCCESS)
    ==> true)
}