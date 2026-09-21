pub open spec fn sdei_interrupt_bind_spec(interrupt: UInt32, result: Result<int64, SdeIStatusCode>, old_s: S, new_s: S) -> bool {
  (result == SDEI_ERROR_NOT_SUPPORTED ==> result == SDEI_ERROR_NOT_SUPPORTED)
  && (result == SDEI_ERROR_INVALID_PARAMETERS ==> result == SDEI_ERROR_INVALID_PARAMETERS)
  && (result == SDEI_ERROR_DENIED ==> result == SDEI_ERROR_DENIED)
  && (result == SDEI_ERROR_OUT_OF_RESOURCE ==> result == SDEI_ERROR_OUT_OF_RESOURCE)
  && ((!(result == SDEI_ERROR_NOT_SUPPORTED) &&
       !(result == SDEI_ERROR_INVALID_PARAMETERS) &&
       !(result == SDEI_ERROR_DENIED) &&
       !(result == SDEI_ERROR_OUT_OF_RESOURCE))
    ==> result == SDEI_SUCCESS)
  && (result != SDEI_SUCCESS
    ==> result == SDEI_ERROR_NOT_SUPPORTED)
  && (result != SDEI_SUCCESS
    ==> result == SDEI_ERROR_INVALID_PARAMETERS)
  && (result != SDEI_SUCCESS
    ==> result == SDEI_ERROR_DENIED)
  && (result != SDEI_SUCCESS
    ==> result == SDEI_ERROR_OUT_OF_RESOURCE)
}