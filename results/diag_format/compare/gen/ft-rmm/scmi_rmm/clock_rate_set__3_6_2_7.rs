pub open spec fn clock_rate_set__3_6_2_7_spec(flags: uint32, clock_id: uint32, rate: [uint32; 2], result: Result<int32, RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!ClockExists(old_s, clock_id) ==> ResultEqual(result, NOT_FOUND))
  && (!ClockSupportsRate(old_s, clock_id, rate) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsValidClockRateSetFlags(old_s, flags) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (flags[0] == 1 && PendingAsyncClockRateChanges(old_s) >= MaxPendingAsyncClockRateChanges(old_s) ==> ResultEqual(result, BUSY))
  && (ClockRateBlockedByDependencies(old_s, clock_id) ==> ResultEqual(result, DENIED))
  && (result.is_Ok() ==> ResultEqual(result, SUCCESS))
  && (result.is_Ok() && flags[0] == 0 implies ClockRate(new_s, clock_id) == RoundRate(new_s, clock_id, rate[3:2]))
  && (result.is_Ok() && flags[0] == 0 && !ClockEnabled(old_s, clock_id) implies the new rate takes effect when the clock is re-enabled)
  && (result.is_Ok() && flags[0] == 1 implies CommandQueued(new_s, CLOCK_RATE_SET, clock_id, rate))
  && (result.is_Ok() && flags[0] == 1 && flags[1] == 0 implies the platform sends CLOCK_RATE_SET_COMPLETE on completion)
  && (result.is_Ok() && flags[0] == 1 && flags[1] == 1 implies the platform does not send a CLOCK_RATE_SET delayed response)
  && ((ClockExists(old_s, clock_id) &&
       ClockSupportsRate(old_s, clock_id, rate) &&
       IsValidClockRateSetFlags(old_s, flags) &&
       !(flags[0] == 1 && PendingAsyncClockRateChanges(old_s) >= MaxPendingAsyncClockRateChanges(old_s)) &&
       !ClockRateBlockedByDependencies(old_s, clock_id))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> ClockRate(new_s, clock_id) == ClockRate(old_s, clock_id))
}