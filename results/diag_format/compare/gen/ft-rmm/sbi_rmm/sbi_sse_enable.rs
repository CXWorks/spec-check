pub open spec fn sbi_sse_enable_spec(event_id: UInt32, result: SbiReturnCode, old_s: S, new_s: S) -> bool {
  (!IsReservedEventId(old_s, event_id) && IsValidEventId(old_s, event_id) && !PlatformSupportsEvent(old_s, event_id) ==> ResultEqual(result, SBI_ERR_NOT_SUPPORTED))
  && (!IsValidEventId(old_s, event_id) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (IsValidEventId(old_s, event_id) && EventState(old_s, event_id) != REGISTERED ==> ResultEqual(result, SBI_ERR_INVALID_STATE))
  && (result == SBI_SUCCESS ==> ResultEqual(result, SBI_SUCCESS))
  && (result == SBI_SUCCESS && IsLocalEvent(old_s, event_id) ==> EventState(new_s, event_id, CallingHart(new_s)) == ENABLED)
  && (result == SBI_SUCCESS && IsGlobalEvent(old_s, event_id) ==> EventState(new_s, event_id) == ENABLED for all harts)
  && ((!(IsReservedEventId(old_s, event_id) && IsValidEventId(old_s, event_id) && !PlatformSupportsEvent(old_s, event_id)) &&
       IsValidEventId(old_s, event_id) &&
       !(IsValidEventId(old_s, event_id) && EventState(old_s, event_id) != REGISTERED))
    ==> result == SBI_SUCCESS)
  && (result != SBI_SUCCESS
    ==> EventState(new_s, event_id, CallingHart(new_s)) == EventState(old_s, event_id, CallingHart(old_s)))
  && (result != SBI_SUCCESS
    ==> EventState(new_s, event_id) == EventState(old_s, event_id))
}