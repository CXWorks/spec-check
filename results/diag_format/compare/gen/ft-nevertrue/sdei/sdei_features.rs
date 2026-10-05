pub open spec fn sdei_features_spec(feature: UInt32, result: Result<int64, SdeiStatusCode>, old_s: S, new_s: S) -> bool {
  (result == SDEI_NOT_SUPPORTED ==> result == SDEI_NOT_SUPPORTED)
  && (result == SDEI_INVALID_PARAMETERS ==> result == SDEI_INVALID_PARAMETERS)
  && ((!(result == SDEI_NOT_SUPPORTED) &&
       !(result == SDEI_INVALID_PARAMETERS))
    ==> result == SDEI_SUCCESS)
}