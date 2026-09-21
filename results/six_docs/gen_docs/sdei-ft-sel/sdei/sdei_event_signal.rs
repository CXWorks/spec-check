pub open spec fn sdei_event_signal_spec(event: int32, target_pe: UInt64, result: SdeiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == SDEI_SUCCESS)
  && (result == NOT_SUPPORTED)
  && (result == INVALID_PARAMETERS)
  && ((!(result == SDEI_SUCCESS) &&
       !(result == NOT_SUPPORTED) &&
       !(result == INVALID_PARAMETERS))
    ==> SdeiEventPending(new_s, target_pe) == SdeiEventPending(old_s, target_pe))
}