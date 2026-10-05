pub open spec fn clock_parent_get__3_6_2_16_spec(clock_id: uint32, status: int32, parent_id: uint32, old_s: S, new_s: S) -> bool {
  (!ClockExists(old_s, clock_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsRequestSupported(old_s, clock_id) ==> ResultEqual(status, NOT_SUPPORTED))
  && (!AgentMayGetClockParent(old_s, calling_agent, clock_id) ==> ResultEqual(status, DENIED))
  && (ResultEqual(status, SUCCESS) ==> parent_id == ClockParent(old_s, clock_id))
  && ((!(ClockExists(old_s, clock_id)) &&
       IsRequestSupported(old_s, clock_id) &&
       AgentMayGetClockParent(old_s, calling_agent, clock_id))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> parent_id == 0)
}