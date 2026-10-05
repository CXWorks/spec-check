pub open spec fn clock_possible_parents_get__3_6_2_14_spec(clock_id: UInt32, skip_parents: UInt32, status: Int32, flags: UInt32, possible_parents: [UInt32; 1], calling_agent: CallingAgent, old_s: S, new_s: S) -> bool {
  (!ClockExists(old_s, clock_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsValidSkipParents(old_s, clock_id, skip_parents) ==> ResultEqual(status, OUT_OF_RANGE))
  && (!PossibleParentsAdvertisementSupported(old_s, clock_id) ==> ResultEqual(status, NOT_SUPPORTED))
  && (!AgentMayGetPossibleParents(old_s, calling_agent, clock_id) ==> ResultEqual(status, DENIED))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> Bits(flags, 23, 8) == 0)
  && (ResultEqual(status, SUCCESS) ==> Bits(flags, 7, 0) == ArrayLength(possible_parents))
  && (ResultEqual(status, SUCCESS) ==> Bits(flags, 31, 24) == RemainingPossibleParents(old_s, clock_id, skip_parents, Bits(flags, 7, 0) as int))
  && (ResultEqual(status, SUCCESS) ==> forall i: UInt32, i < Bits(flags, 7, 0) ==> ElementAt(possible_parents, i) == NthPossibleParent(old_s, clock_id, skip_parents + i as int))
  && (ResultEqual(status, SUCCESS) ==> forall i: UInt32, i + 1 < Bits(flags, 7, 0) ==> ElementAt(possible_parents, i) < ElementAt(possible_parents, i + 1))
  && ((ClockExists(old_s, clock_id) &&
       IsValidSkipParents(old_s, clock_id, skip_parents) &&
       PossibleParentsAdvertisementSupported(old_s, clock_id) &&
       AgentMayGetPossibleParents(old_s, calling_agent, clock_id))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> Bits(flags, 23, 8) == 0)
  && (result != SUCCESS
    ==> Bits(flags, 7, 0) == ArrayLength(possible_parents))
  && (result != SUCCESS
    ==> Bits(flags, 31, 24) == RemainingPossibleParents(old_s, clock_id, skip_parents, Bits(flags, 7, 0) as int))
  && (result != SUCCESS
    ==> forall i: UInt32, i < Bits(flags, 7, 0) ==> ElementAt(possible_parents, i) == NthPossibleParent(old_s, clock_id, skip_parents + i as int))
  && (result != SUCCESS
    ==> forall i: UInt32, i + 1 < Bits(flags, 7, 0) ==> ElementAt(possible_parents, i) < ElementAt(possible_parents, i + 1))
}