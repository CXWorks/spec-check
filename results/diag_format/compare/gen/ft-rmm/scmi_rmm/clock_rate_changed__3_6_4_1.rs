pub open spec fn clock_rate_changed__3_6_4_1_spec(agent_id: UInt32, clock_id: UInt32, rate: [UInt32; 2], old_s: S, new_s: S) -> bool {
  (IsRegisteredForClockRateChangeNotification(old_s, recipient_agent, clock_id) ==> NotificationSentTo(new_s, recipient_agent))
  && (ClockRateTransitionComplete(new_s, clock_id))
  && (((rate[1] << 32) | rate[0]) == ClockRate(new_s, clock_id))
  && (agent_id == AgentThatCausedRateChange(new_s, clock_id))
  && ((!(IsRegisteredForClockRateChangeNotification(old_s, recipient_agent, clock_id)))
    ==> NotificationSentTo(new_s, recipient_agent))
  && (ClockRateTransitionComplete(new_s, clock_id))
  && (((rate[1] << 32) | rate[0]) == ClockRate(new_s, clock_id))
  && (agent_id == AgentThatCausedRateChange(new_s, clock_id))
}