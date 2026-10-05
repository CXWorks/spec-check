pub open spec fn clock_rate_change_requested__3_6_4_2_spec(agent_id: UInt32, domain_id: UInt32, rate_lower: UInt32, rate_upper: UInt32, old_s: S, new_s: S) -> bool {
  (forall (a: Agent), IsSubscribedToClockRateChangeRequested(new_s, a, domain_id) ==> NotificationDelivered(new_s, a, agent_id, domain_id, rate_lower, rate_upper))
  && (RequestedClockRate(new_s, agent_id, domain_id) == rate_lower + rate_upper * 4294967296)
  && ((!(forall (a: Agent), IsSubscribedToClockRateChangeRequested(old_s, a, domain_id)) &&
       RequestedClockRate(old_s, agent_id, domain_id) != rate_lower + rate_upper * 4294967296)
    ==> RequestedClockRate(new_s, agent_id, domain_id) == rate_lower + rate_upper * 4294967296)
}