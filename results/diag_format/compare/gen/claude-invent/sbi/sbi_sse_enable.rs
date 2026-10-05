pub open spec fn sbi_sse_enable_spec(result: SbiError, event_id: UInt32, old_s: S, new_s: S) -> bool {
    (!SseEventIdValid(old_s, event_id) ==> result == SBI_ERR_INVALID_PARAM)
    && ((SseEventIdValid(old_s, event_id) && !SseEventSupported(old_s, event_id)) ==> result == SBI_ERR_NOT_SUPPORTED)
    && ((SseEventIdValid(old_s, event_id)
        && SseEventSupported(old_s, event_id)
        && SseEventState(old_s, event_id, CallingHart(old_s)) != REGISTERED) ==> result == SBI_ERR_INVALID_STATE)
    && ((SseEventIdValid(old_s, event_id)
        && SseEventSupported(old_s, event_id)
        && SseEventState(old_s, event_id, CallingHart(old_s)) == REGISTERED) ==> (
            result == SBI_SUCCESS
            && (SseEventIsGlobal(old_s, event_id) ==> (forall|h: HartId| IsHart(old_s, h) ==> SseEventState(new_s, event_id, h) == ENABLED))
            && (!SseEventIsGlobal(old_s, event_id) ==> (
                SseEventState(new_s, event_id, CallingHart(old_s)) == ENABLED
                && (forall|h: HartId| (IsHart(old_s, h) && h != CallingHart(old_s)) ==> SseEventState(new_s, event_id, h) == SseEventState(old_s, event_id, h))
            ))
            && (forall|e: UInt32, h: HartId| e != event_id ==> SseEventState(new_s, e, h) == SseEventState(old_s, e, h))
        ))
    && (result != SBI_SUCCESS ==> new_s == old_s)
}
