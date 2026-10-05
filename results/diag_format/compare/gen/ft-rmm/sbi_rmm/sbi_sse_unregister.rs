pub open spec fn sbi_sse_unregister_spec(event_id: uint32_t, result: SbiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (SseEventState(old_s, event_id) != REGISTERED ==> result == RSI_ERROR_STATE)
  && (result == RSI_SUCCESS && IsLocalEvent(old_s, event_id) ==> SseEventState(new_s, event_id) == UNUSED)
  && (result == RSI_SUCCESS && IsGlobalEvent(old_s, event_id) ==> SseEventState(new_s, event_id) == UNUSED)
  && (result == RSI_SUCCESS && IsLocalEvent(old_s, event_id) ==> !IsHandlerRegistered(new_s, event_id))
  && (result == RSI_SUCCESS && IsGlobalEvent(old_s, event_id) ==> !IsHandlerRegistered(new_s, event_id))
  && ((!(SseEventState(old_s, event_id) != REGISTERED))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> SseEventState(new_s, event_id) == SseEventState(old_s, event_id))
  && (result != RSI_SUCCESS
    ==> SseEventState(new_s, event_id) == SseEventState(old_s, event_id))
  && (result != RSI_SUCCESS
    ==> SseEventState(new_s, event_id) == SseEventState(old_s, event_id))
  && (result != RSI_SUCCESS
    ==> SseEventState(new_s, event_id) == SseEventState(old_s, event_id))
  && (result != RSI_SUCCESS
    ==> SseEventState(new_s, event_id) == SseEventState(old_s, event_id))
}