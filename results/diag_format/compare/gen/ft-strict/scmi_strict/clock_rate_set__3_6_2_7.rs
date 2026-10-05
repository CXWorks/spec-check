pub open spec fn clock_rate_set__3_6_2_7_spec(flags: UInt32, clock_id: UInt32, rate: [UInt32; 2], status: Int32, old_s: S, new_s: S) -> bool {
  (!ClockExists(old_s, clock_id) ==> ResultEqual(status, NOT_FOUND))
  && (!ClockSupportsRate(old_s, clock_id, RequestedRate(rate)) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!IsValidClockRateSetFlags(old_s, flags) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (Bits(flags, 0, 0) == 1 && PendingAsyncClockRateChanges(old_s) >= MaxPendingAsyncClockRateChanges(old_s) ==> ResultEqual(status, BUSY))
  && (ClockRateChangeBlockedByDependencies(old_s, clock_id) ==> ResultEqual(status, DENIED))
  && (ResultEqual(status, SUCCESS) ==> (Bits(flags, 0, 0) == 0 && ClockIsEnabled(old_s, clock_id) ==> ClockRate(new_s, clock_id) == SelectPhysicalRate(new_s, clock_id, RequestedRate(rate), Bits(flags, 3, 2 as int))))
  && (ResultEqual(status, SUCCESS) ==> (Bits(flags, 0, 0) == 0 && !ClockIsEnabled(old_s, clock_id) ==> RateTakesEffectOnReEnable(new_s, clock_id, SelectPhysicalRate(new_s, clock_id, RequestedRate(rate), Bits(flags, 3, 2 as int))))
  && (ResultEqual(status, SUCCESS) ==> (Bits(flags, 0, 0) == 1 ==> AsyncClockRateChangeQueued(new_s, clock_id, SelectPhysicalRate(new_s, clock_id, RequestedRate(rate), Bits(flags, 3, 2 as int))))
  && (ResultEqual(status, SUCCESS) ==> (Bits(flags, 0, 0) == 1 && Bits(flags, 1, 1) == 0 ==> DelayedResponseSent(new_s, CLOCK_RATE_SET_COMPLETE, clock_id)))
  && (ResultEqual(status, SUCCESS) ==> (Bits(flags, 0, 0) == 1 && Bits(flags, 1, 1) == 1 ==> !DelayedResponseSent(new_s, CLOCK_RATE_SET_COMPLETE, clock_id)))
  && ((ClockExists(old_s, clock_id) &&
       ClockSupportsRate(old_s, clock_id, RequestedRate(rate)) &&
       IsValidClockRateSetFlags(old_s, flags) &&
       !(Bits(flags, 0, 0) == 1 && PendingAsyncClockRateChanges(old_s) >= MaxPendingAsyncClockRateChanges(old_s)) &&
       !ClockRateChangeBlockedByDependencies(old_s, clock_id))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> ClockRate(new_s, clock_id) == ClockRate(old_s, clock_id))
  && (result != SUCCESS
    ==> RateTakesEffectOnReEnable(new_s, clock_id, SelectPhysicalRate(new_s, clock_id, RequestedRate(rate), Bits(flags, 3, 2 as int))) == RateTakesEffectOnReEnable(old_s, clock_id, SelectPhysicalRate(old_s, clock_id, RequestedRate(rate), Bits(flags, 3, 2 as int))))
  && (result != SUCCESS
    ==> AsyncClockRateChangeQueued(new_s, clock_id, SelectPhysicalRate(new_s, clock_id, RequestedRate(rate), Bits(flags, 3, 2 as int))) == AsyncClockRateChangeQueued(old_s, clock_id, SelectPhysicalRate(old_s, clock_id, RequestedRate(rate), Bits(flags, 3, 2 as int))))
  && (result != SUCCESS
    ==> DelayedResponseSent(new_s, CLOCK_RATE_SET_COMPLETE, clock_id) == DelayedResponseSent(old_s, CLOCK_RATE_SET_COMPLETE, clock_id))
  && (result != SUCCESS
    ==> !DelayedResponseSent(new_s, CLOCK_RATE_SET_COMPLETE, clock_id) == !DelayedResponseSent(old_s, CLOCK_RATE_SET_COMPLETE, clock_id))
}