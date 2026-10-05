pub open spec fn sbi_sse_enable_spec(result: SbiReturnCode, old_s: S, new_s: S) -> bool {
    (!IsReservedEventId(old_s, event_id) && IsValidEventId(old_s, event_id) && !PlatformSupportsEvent(old_s, event_id) ==> ResultEqual(result, SBI_ERR_NOT_SUPPORTED))
    && (!IsValidEventId(old_s, event_id) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (IsValidEventId(old_s, event_id) && EventState(old_s, event_id) != REGISTERED ==> ResultEqual(result, SBI_ERR_INVALID_STATE))
    && (ResultEqual(result, SBI_SUCCESS) ==> (IsLocalEvent(old_s, event_id) ==> EventState(new_s, event_id, CallingHart()) == ENABLED && IsGlobalEvent(old_s, event_id) ==> EventState(new_s, event_id) == ENABLED))
}