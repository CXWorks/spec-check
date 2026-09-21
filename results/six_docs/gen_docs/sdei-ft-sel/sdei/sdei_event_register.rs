pub open spec fn sdei_event_register_spec(event: int, entry_point_address: UInt64, ep_argument: UInt64, flags: UInt64, affinity: UInt64, result: SdeCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == SDEI_SUCCESS ==> true)
  && (result == SDEI_NOT_SUPPORTED ==> true)
  && (result == SDEI_INVALID_PARAMETERS ==> true)
  && (result == SDEI_DENIED ==> true)
  && ((!(result == SDEI_SUCCESS) &&
       !(result == SDEI_NOT_SUPPORTED) &&
       !(result == SDEI_INVALID_PARAMETERS) &&
       !(result == SDEI_DENIED))
    ==> true)
}