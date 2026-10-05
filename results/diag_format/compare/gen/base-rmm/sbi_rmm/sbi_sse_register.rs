pub open spec fn sbi_sse_register_spec(result: SbiErrorCode, old_s: S, new_s: S) -> bool {
    (!IsValidEventId(event_id) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (!IsAligned(handler_entry_pc, 2) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (!PlatformSupportsEvent(event_id) ==> ResultEqual(result, SBI_ERR_NOT_SUPPORTED))
    && ((IsLocalEvent(event_id) && SseEvent(event_id, CallingHart()).state != UNUSED) || (IsGlobalEvent(event_id) && SseEvent(event_id).state != UNUSED) ==> ResultEqual(result, SBI_ERR_INVALID_STATE))
    && (ResultEqual(result, SBI_SUCCESS) ==> (IsLocalEvent(event_id) ==> SseEvent(event_id, CallingHart()).state == REGISTERED))
    && (ResultEqual(result, SBI_SUCCESS) ==> (IsGlobalEvent(event_id) ==> SseEvent(event_id).state == REGISTERED))
    && (ResultEqual(result, SBI_SUCCESS) ==> SseEvent(event_id).attr.ENTRY_PC == handler_entry_pc)
    && (ResultEqual(result, SBI_SUCCESS) ==> SseEvent(event_id).attr.ENTRY_ARG == handler_entry_arg)
}