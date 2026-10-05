pub open spec fn sbi_set_timer_spec(stime_value: UInt64, error: SbiReturnCode, old_s: S, new_s: S) -> bool {
  (ResultEqual(error, SBI_SUCCESS) ==> NextTimerEventTime(new_s) == stime_value)
  && (ResultEqual(error, SBI_SUCCESS) && IsTimeInFuture(old_s, stime_value) ==> !TimerInterruptPending(new_s))
  && ((!(ResultEqual(error, SBI_SUCCESS)))
    ==> NextTimerEventTime(new_s) == NextTimerEventTime(old_s))
  && ((!(ResultEqual(error, SBI_SUCCESS)))
    ==> TimerInterruptPending(new_s) == TimerInterruptPending(old_s))
}