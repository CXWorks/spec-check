pub open spec fn clock_possible_parents_get__3_6_2_14_spec(result: Int32, flags: UInt32, possible_parents: Array<UInt32>, old_s: S, new_s: S) -> bool {
    (!ClockExists(old_s, clock_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (!IsValidParentSkip(old_s, clock_id(old_s), skip_parents(old_s)) ==> ResultEqual(result, OUT_OF_RANGE))
    && (!IsPossibleParentsQuerySupported(old_s, clock_id(old_s)) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!AgentMayGetPossibleParents(old_s, calling_agent(old_s), clock_id(old_s)) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> (flags[7:0] == N(old_s, clock_id(old_s), skip_parents(old_s)) && flags[31:24] == RemainingPossibleParents(old_s, clock_id(old_s), skip_parents(old_s), N(old_s, clock_id(old_s), skip_parents(old_s))) && flags[23:8] == 0 && possible_parents[0] == PossibleParents(old_s, clock_id(old_s))[skip_parents(old_s)] && IsAscendingOrder(possible_parents[0..N(old_s, clock_id(old_s), skip_parents(old_s))-1])))
}