pub open spec fn sbi_sse_register_spec(result: SbiReturnCode, old_s: S, new_s: S) -> bool {
    (!IsValidEventId(event_id) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (handler_entry_pc % 2 != 0 ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (IsReservedEventId(event_id) && IsValidEventId(event_id) && !PlatformSupportsEvent(event_id) ==> ResultEqual(result, SBI_ERR_NOT_SUPPORTED))
    && (EventState(event_id) != UNUSED ==> ResultEqual(result, SBI_ERR_INVALID_STATE))
    && (ResultEqual(result, SBI_SUCCESS) ==> (IsLocalEvent(event_id) ==> EventStateForHart(event_id, CallingHart()) == REGISTERED))
    && (ResultEqual(result, SBI_SUCCESS) ==> (IsGlobalEvent(event_id) ==> forall|h: Hart| EventStateForHart(event_id, h) == REGISTERED))
    && (ResultEqual(result, SBI_SUCCESS) ==> EventAttribute(event_id, ENTRY_PC) == handler_entry_pc)
    && (ResultEqual(result, SBI_SUCCESS) ==> EventAttribute(event_id, ENTRY_ARG) == handler_entry_arg)
}