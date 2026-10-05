pub open spec fn sbi_sse_complete_spec(result: SbiRet, old_s: S, new_s: S) -> bool {
    (!SseHartHasRunningEvent(old_s) ==> (result.error == SBI_SUCCESS && new_s == old_s))
    && (SseHartHasRunningEvent(old_s) ==> {
        let ev = SseHighestPriorityRunningEvent(old_s);
        (SseEventIsOneShot(old_s, ev) ==> SseEventState(new_s, ev) == SSE_STATE_REGISTERED)
        && (!SseEventIsOneShot(old_s, ev) ==> SseEventState(new_s, ev) == SSE_STATE_ENABLED)
        && SseInterruptedSupervisorStateResumed(old_s, new_s, ev)
    })
}
