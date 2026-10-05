pub open spec fn clock_possible_parents_get__3_6_2_14_spec(clock_id: UInt32, skip_parents: UInt32, status: Int32, flags: UInt32, possible_parents: [UInt32; 1], old_s: S, new_s: S) -> bool {
  (!ClockExists(old_s, clock_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsValidParentSkip(old_s, clock_id, skip_parents) ==> ResultEqual(status, OUT_OF_RANGE))
  && (!IsPossibleParentsQuerySupported(old_s, clock_id) ==> ResultEqual(status, NOT_SUPPORTED))
  && (!AgentMayGetPossibleParents(old_s, calling_agent, clock_id) ==> ResultEqual(status, DENIED))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> flags[7:0] == N)
  && (ResultEqual(status, SUCCESS) ==> flags[31:24] == RemainingPossibleParents(new_s, clock_id, skip_parents, N))
  && (ResultEqual(status, SUCCESS) ==> flags[23:8] == 0)
  && (ResultEqual(status, SUCCESS) ==> possible_parents[0] == PossibleParents(new_s, clock_id)[skip_parents])
  && (ResultEqual(status, SUCCESS) ==> IsAscendingOrder(new_s, possible_parents[0..N-1]))
  && ((ClockExists(old_s, clock_id) &&
       IsValidParentSkip(old_s, clock_id, skip_parents) &&
       IsPossibleParentsQuerySupported(old_s, clock_id) &&
       AgentMayGetPossibleParents(old_s, calling_agent, clock_id))
    ==> ResultEqual(status, SUCCESS))
}