pub open spec fn sbi_sse_register_spec(event_id: UInt32, handler_entry_pc: Address, handler_entry_arg: UInt, result: SbiErrorCode, old_s: S, new_s: S) -> bool {
  (!IsValidEventId(old_s, event_id) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (!IsAligned(old_s, handler_entry_pc, 2) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (!PlatformSupportsEvent(old_s, event_id) ==> ResultEqual(result, SBI_ERR_NOT_SUPPORTED))
  && ((IsLocalEvent(old_s, event_id) && SseEvent(old_s, event_id, CallingHart(old_s)).state != UNUSED) || (IsGlobalEvent(old_s, event_id) && SseEvent(old_s, event_id).state != UNUSED) ==> ResultEqual(result, SBI_ERR_INVALID_STATE))
  && (result == SBI_SUCCESS ==> SseEvent(new_s, event_id, CallingHart(new_s)).state == REGISTERED)
  && (result == SBI_SUCCESS && IsLocalEvent(old_s, event_id) ==> SseEvent(new_s, event_id, CallingHart(new_s)).state == REGISTERED)
  && (result == SBI_SUCCESS && IsGlobalEvent(old_s, event_id) ==> SseEvent(new_s, event_id).state == REGISTERED)
  && (result == SBI_SUCCESS ==> SseEvent(new_s, event_id).attr.ENTRY_PC == handler_entry_pc)
  && (result == SBI_SUCCESS ==> SseEvent(new_s, event_id).attr.ENTRY_ARG == handler_entry_arg)
  && ((IsValidEventId(old_s, event_id) &&
       IsAligned(old_s, handler_entry_pc, 2) &&
       PlatformSupportsEvent(old_s, event_id) &&
       !((IsLocalEvent(old_s, event_id) && SseEvent(old_s, event_id, CallingHart(old_s)).state != UNUSED) ||
        (IsGlobalEvent(old_s, event_id) && SseEvent(old_s, event_id).state != UNUSED)))
    ==> result == SBI_SUCCESS)
  && (result != SBI_SUCCESS
    ==> SseEvent(new_s, event_id, CallingHart(new_s)).state == SseEvent(old_s, event_id, CallingHart(old_s)).state)
  && (result != SBI_SUCCESS
    ==> SseEvent(new_s, event_id).state == SseEvent(old_s, event_id).state)
  && (result != SBI_SUCCESS
    ==> SseEvent(new_s, event_id).attr.ENTRY_PC == SseEvent(old_s, event_id).attr.ENTRY_PC)
  && (result != SBI_SUCCESS
    ==> SseEvent(new_s, event_id).attr.ENTRY_ARG == SseEvent(old_s, event_id).attr.ENTRY_ARG)
}