pub open spec fn sbi_sse_register_spec(result: SbiErrorCode, event_id: UInt32, handler_entry_pc: UInt64, handler_entry_arg: UInt64, old_s: S, new_s: S) -> bool {
    ((!SseEventIdIsValid(event_id) || (handler_entry_pc as int) % 2 != 0) ==> result == SBI_ERR_INVALID_PARAM)
    && ((SseEventIdIsValid(event_id) && (handler_entry_pc as int) % 2 == 0 && !SseEventIsSupported(old_s, event_id)) ==> result == SBI_ERR_NOT_SUPPORTED)
    && ((SseEventIdIsValid(event_id) && (handler_entry_pc as int) % 2 == 0 && SseEventIsSupported(old_s, event_id) && SseEventState(old_s, CallingHart(old_s), event_id) != SSE_STATE_UNUSED) ==> result == SBI_ERR_INVALID_STATE)
    && ((result != SBI_SUCCESS) ==> new_s == old_s)
    && ((SseEventIdIsValid(event_id) && (handler_entry_pc as int) % 2 == 0 && SseEventIsSupported(old_s, event_id) && SseEventState(old_s, CallingHart(old_s), event_id) == SSE_STATE_UNUSED) ==> (
        result == SBI_SUCCESS
        && (!SseEventIsGlobal(event_id) ==> (
            SseEventState(new_s, CallingHart(old_s), event_id) == SSE_STATE_REGISTERED
            && SseEventEntryPc(new_s, CallingHart(old_s), event_id) == handler_entry_pc
            && SseEventEntryArg(new_s, CallingHart(old_s), event_id) == handler_entry_arg
            && (forall|h: Hart| h != CallingHart(old_s) ==> SseEventState(new_s, h, event_id) == SseEventState(old_s, h, event_id))
        ))
        && (SseEventIsGlobal(event_id) ==> (forall|h: Hart| IsValidHart(old_s, h) ==> (
            SseEventState(new_s, h, event_id) == SSE_STATE_REGISTERED
            && SseEventEntryPc(new_s, h, event_id) == handler_entry_pc
            && SseEventEntryArg(new_s, h, event_id) == handler_entry_arg
        )))
        && (forall|h: Hart, e: UInt32| e != event_id ==> SseEventState(new_s, h, e) == SseEventState(old_s, h, e))
    ))
}
