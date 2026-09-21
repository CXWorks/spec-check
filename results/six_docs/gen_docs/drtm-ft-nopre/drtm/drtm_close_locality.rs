pub open spec fn drtm_close_locality_spec(locality: UInt32, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (result.is_Err() ==> result == RMI_ERROR_NOT_SUPPORTED)
  && (result.is_Err() ==> result == RMI_ERROR_INVALID_PARAMETERS)
  && (result.is_Err() ==> result == RMI_ERROR_ALREADY_CLOSED)
  && (result.is_Err() ==> result == RMI_ERROR_DENIED)
  && ((!(locality == 2 || locality == 3)) ==> result == RMI_ERROR_INVALID_PARAMETERS)
  && (result.is_Ok()
    ==> true)
}