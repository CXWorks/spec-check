pub open spec fn sbi_sse_enable_spec(event_id: UInt32, result: SbiErrorCode, old_s: S, new_s: S) -> bool {
  (!IsValidEventId(old_s, event_id) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (IsValidEventId(old_s, event_id) && !IsReservedEventId(old_s, event_id) && !PlatformSupportsEvent(old_s, event_id) ==> ResultEqual(result, SBI_ERR_NOT_SUPPORTED))
  && (IsValidEventId(old_s, event_id) && EventAt(old_s, event_id).state != REGISTERED ==> ResultEqual(result, SBI_ERR_INVALID_STATE))
  && (result == SBI_SUCCESS ==> EventAt(new_s, event_id).state == ENABLED)
  && (result == SBI_SUCCESS && IsLocalEvent(old_s, event_id) ==> EventEnabledOnHart(new_s, event_id, CallingHart(new_s)))
  && (result == SBI_SUCCESS && IsGlobalEvent(old_s, event_id) ==> (forall (h: Hart), EventEnabledOnHart(new_s, event_id, h)))
  && ((IsValidEventId(old_s, event_id) && IsReservedEventId(old_s, event_id) && PlatformSupportsEvent(old_s, event_id))
    ==> ResultEqual(result, SBI_SUCCESS))
  && (result != SBI_SUCCESS
    ==> EventAt(new_s, event_id).state == EventAt(old_s, event_id).state)
  && (result != SBI_SUCCESS
    ==> EventAt(new_s, event_id).state == EventAt(old_s, event_id).state)
  && (!(IsValidEventId(old_s, event_id) && IsReservedEventId(old_s, event_id) && !PlatformSupportsEvent(old_s, event_id))
    ==> ResultEqual(result, SBI_SUCCESS))
}