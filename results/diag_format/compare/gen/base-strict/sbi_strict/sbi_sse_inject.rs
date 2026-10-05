pub open spec fn sbi_sse_inject_spec(result: SbiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (EventAttributeAllowsInjection(old_s, old_s.cmd_input_event_id) ==> ResultEqual(result, SBI_SUCCESS))
    && (IsLocalEvent(old_s, old_s.cmd_input_event_id) ==> (ResultEqual(result, SBI_SUCCESS) ==> EventInjectedOnHart(old_s, old_s.cmd_input_event_id, old_s.cmd_input_hart_id)))
    && (IsGlobalEvent(old_s, old_s.cmd_input_event_id) ==> (ResultEqual(result, SBI_SUCCESS) ==> EventInjected(old_s, old_s.cmd_input_event_id)))
    && (InSseEventHandler(old_s) && EventReadyToRun(old_s, old_s.cmd_input_event_id) && EventPriority(old_s, old_s.cmd_input_event_id) > EventPriority(old_s, RunningSseEvent(old_s)) ==> EventHandledImmediately(old_s, old_s.cmd_input_event_id))
    && (InSseEventHandler(old_s) && EventReadyToRun(old_s, old_s.cmd_input_event_id) && EventPriority(old_s, old_s.cmd_input_event_id) < EventPriority(old_s, RunningSseEvent(old_s)) ==> EventRunsAfterCompletionOf(old_s, old_s.cmd_input_event_id, RunningSseEvent(old_s)))
    && (SseEventAt(old_s, old_s.cmd_input_event_id, old_s.cmd_input_hart_id).injected == true)
}