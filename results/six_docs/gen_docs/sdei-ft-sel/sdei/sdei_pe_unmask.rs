pub open spec fn sdei_pe_unmask_spec(result: int64, old_s: S, new_s: S) -> bool {
  (result == SDEI_SUCCESS)
  && (result == SDEI_NOT_SUPPORTED)
  && ((!(result == SDEI_SUCCESS) &&
       !(result == SDEI_NOT_SUPPORTED))
    ==> true)
}