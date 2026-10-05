pub open spec fn clock_rate_set__3_6_2_7_spec(result: int32, old_s: S, new_s: S) -> bool {
    (!ClockExists(old_s, clock_id) ==> ResultEqual(result, NOT_FOUND))
    && (!ClockSupportsRate(old_s, clock_id, rate) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsValidClockRateSetFlags(old_s, flags) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (flags[0] == 1 && PendingAsyncClockRateChanges(old_s) >= MaxPendingAsyncClockRateChanges() ==> ResultEqual(result, BUSY))
    && (ClockRateBlockedByDependencies(old_s, clock_id) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> (flags[0] == 0 implies ClockRate(new_s, clock_id) == RoundRate(old_s, clock_id, rate, flags[3:2])))
    && (flags[0] == 0 && !ClockEnabled(old_s, clock_id) ==> (the new rate takes effect when the clock is re-enabled))
    && (flags[0] == 1 ==> CommandQueued(CLOCK_RATE_SET, clock_id, rate))
    && (flags[0] == 1 && flags[1] == 0 ==> the platform sends CLOCK_RATE_SET_COMPLETE on completion)
    && (flags[0] == 1 && flags[1] == 1 ==> the platform does not send a CLOCK_RATE_SET delayed response)
}