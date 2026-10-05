pub open spec fn clock_rate_set__3_6_2_7_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (!ClockExists(old_s, clock_id) ==> ResultEqual(result, NOT_FOUND))
    && (!ClockSupportsRate(old_s, clock_id, RequestedRate(rate)) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsValidClockRateSetFlags(old_s, flags) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (Bits(old_s, flags, 0, 0) == 1 && PendingAsyncClockRateChanges(old_s) >= MaxPendingAsyncClockRateChanges() ==> ResultEqual(result, BUSY))
    && (ClockRateChangeBlockedByDependencies(old_s, clock_id) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> (
        (Bits(old_s, flags, 0, 0) == 0 && ClockIsEnabled(old_s, clock_id) ==> ClockRate(old_s, clock_id) == SelectPhysicalRate(old_s, clock_id, RequestedRate(rate), Bits(old_s, flags, 3, 2)))
        && (Bits(old_s, flags, 0, 0) == 0 && !ClockIsEnabled(old_s, clock_id) ==> RateTakesEffectOnReEnable(old_s, clock_id, SelectPhysicalRate(old_s, clock_id, RequestedRate(rate), Bits(old_s, flags, 3, 2))))
        && (Bits(old_s, flags, 0, 0) == 1 ==> AsyncClockRateChangeQueued(old_s, clock_id, SelectPhysicalRate(old_s, clock_id, RequestedRate(rate), Bits(old_s, flags, 3, 2))))
        && (Bits(old_s, flags, 0, 0) == 1 && Bits(old_s, flags, 1, 1) == 0 ==> DelayedResponseSent(CLOCK_RATE_SET_COMPLETE, clock_id))
        && (Bits(old_s, flags, 0, 0) == 1 && Bits(old_s, flags, 1, 1) == 1 ==> !DelayedResponseSent(CLOCK_RATE_SET_COMPLETE, clock_id))
    ))
}