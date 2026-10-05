pub open spec fn drtm_close_locality_spec(locality: UInt32, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS)
  && (result == RSI_NOT_SUPPORTED)
  && (result == RSI_INVALID_PARAMETERS)
  && (result == RSI_ALREADY_CLOSED)
  && (result == RSI_DENIED)
  && ((!(locality == 2 || locality == 3)) ==> result == RSI_INVALID_PARAMETERS)
}