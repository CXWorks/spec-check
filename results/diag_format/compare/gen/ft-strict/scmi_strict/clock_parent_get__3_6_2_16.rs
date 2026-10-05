pub open spec fn clock_parent_get__3_6_2_16_spec(clock_id: UInt32, status: Int32, parent_id: UInt32, old_s: S, new_s: S) -> bool {
  (!ClockExists(old_s, clock_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsClockParentGetSupported(old_s, clock_id) ==> ResultEqual(status, NOT_SUPPORTED))
  && (!IsAgentAllowedToGetClockParent(old_s, CallingAgent(), clock_id) ==> ResultEqual(status, DENIED))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> parent_id == ClockParent(new_s, clock_id))
  && ((ClockExists(old_s, clock_id) &&
       IsClockParentGetSupported(old_s, clock_id) &&
       IsAgentAllowedToGetClockParent(old_s, CallingAgent(), clock_id))
    ==> ResultEqual(status, SUCCESS))
}