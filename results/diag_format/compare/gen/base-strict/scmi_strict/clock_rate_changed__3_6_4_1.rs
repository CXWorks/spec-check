pub open spec fn clock_rate_changed__3_6_4_1_spec(result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (result.is_Ok())
    && (IsRegisteredForClockRateNotification(old_s, RecipientAgent()) ==> IsRegisteredForClockRateNotification(new_s, RecipientAgent()))
    && (ClockRateChangedByOtherAgentOrPlatform(old_s, agent_id, RecipientAgent()) ==> ClockRateChangedByOtherAgentOrPlatform(new_s, agent_id, RecipientAgent()))
    && (ClockRateTransitionCompleted(old_s, clock_id) ==> ClockRateTransitionCompleted(new_s, clock_id))
    && (ClockRate(old_s, clock_id) == RateLow(rate) + RateHigh(rate) * 4294967296 ==> ClockRate(new_s, clock_id) == RateLow(rate) + RateHigh(rate) * 4294967296)
    && (old_s == new_s)
}