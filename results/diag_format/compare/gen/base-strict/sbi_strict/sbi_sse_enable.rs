pub open spec fn sbi_sse_enable_spec(result: SbiErrorCode, old_s: S, new_s: S) -> bool {
    (!IsValidEventId(old_s, event_id) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (IsValidEventId(old_s, event_id) && !IsReservedEventId(old_s, event_id) && !PlatformSupportsEvent(old_s, event_id) ==> ResultEqual(result, SBI_ERR_NOT_SUPPORTED))
    && (IsValidEventId(old_s, event_id) && EventAt(old_s, event_id).state != REGISTERED ==> ResultEqual(result, SBI_ERR_INVALID_STATE))
    && (ResultEqual(result, SBI_SUCCESS) ==> EventAt(new_s, event_id).state == ENABLED)
    && (IsLocalEvent(event_id) ==> EventEnabledOnHart(new_s, event_id, CallingHart()))
    && (IsGlobalEvent(event_id) ==> forall|h: Hart| EventEnabledOnHart(new_s, event_id, h))
}