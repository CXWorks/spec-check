pub open spec fn sbi_set_timer_spec(stime_value: UInt64, result: SbiRet, old_s: S, new_s: S) -> bool {
    (result.error == SBI_SUCCESS)
    && (TimerCompareValue(new_s) == stime_value)
    && (IsFutureTime(old_s, stime_value) ==> !SupervisorTimerInterruptPending(new_s))
    && (SieStie(new_s) == SieStie(old_s))
}
