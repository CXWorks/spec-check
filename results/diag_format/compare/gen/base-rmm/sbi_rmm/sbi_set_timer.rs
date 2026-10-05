pub open spec fn sbi_set_timer_spec(error: SbiReturnCode, timer_event: u64, timer_pend: bool, old_s: S, new_s: S) -> bool {
    (true ==> ResultEqual(error, SBI_SUCCESS))
    && (true ==> timer_event == NextTimerEventTime())
    && (IsTimeInFuture(stime_value(old_s)) ==> !timer_pend)
}