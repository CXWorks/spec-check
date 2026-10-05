pub open spec fn sdei_interrupt_release_spec(event: int32, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == RSI_ERROR_INVALID_PARAMETERS)
  && (result == RSI_ERROR_DENIED)
  && ((!(result == RSI_SUCCESS)) ==> SdeiEventAt(new_s, event as int).state == SdeiEventState::SDEI_EVENT_UNREGISTERED)
  && (result == RSI_SUCCESS ==> SdeiEventAt(new_s, event as int).state == SdeiEventState::SDEI_EVENT_UNREGISTERED)
  && ((!(result == RSI_SUCCESS)) ==> SdeiEventAt(new_s, event as int).state == SdeiEventAt(old_s, event as int).state)
  && (result != RSI_SUCCESS ==> SdeiEventAt(new_s, event as int).interrupt == SdeiEventAt(old_s, event as int).interrupt)
}