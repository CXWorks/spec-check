pub open spec fn clock_rate_changed__3_6_4_1_spec(agent_id: UInt32, clock_id: UInt32, rate: UInt32[2], old_s: S, new_s: S) -> bool {
  (IsRegisteredForClockRateChangeNotification(new_s, RecipientAgent(new_s), clock_id))
  && (ClockRateChangedByOtherAgentOrPlatform(new_s, agent_id, RecipientAgent(new_s)))
  && (ClockRateTransitionCompleted(new_s, clock_id))
  && (ClockRate(new_s, clock_id) == RateLow(rate) + RateHigh(rate) * 4294967296)
  && ((!(IsRegisteredForClockRateChangeNotification(old_s, RecipientAgent(old_s), clock_id)))
    ==> true)
  && (!(ClockRateChangedByOtherAgentOrPlatform(old_s, agent_id, RecipientAgent(old_s)))
    ==> true)
  && (!(ClockRateTransitionCompleted(old_s, clock_id))
    ==> true)
  && (!(ClockRate(old_s, clock_id) == RateLow(rate) + RateHigh(rate) * 4294967296)
    ==> true)
}