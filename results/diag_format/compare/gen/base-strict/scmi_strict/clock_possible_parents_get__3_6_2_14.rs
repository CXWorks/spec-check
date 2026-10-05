pub open spec fn clock_possible_parents_get__3_6_2_14_spec(result: Int32, flags: UInt32, possible_parents: Array<UInt32>, old_s: S, new_s: S) -> bool {
    (!ClockExists(old_s, clock_id) ==> ResultEqual(result, NOT_FOUND))
    && (!IsValidSkipParents(old_s, clock_id, skip_parents) ==> ResultEqual(result, OUT_OF_RANGE))
    && (!PossibleParentsAdvertisementSupported(old_s, clock_id) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!AgentMayGetPossibleParents(old_s, calling_agent, clock_id) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> (
        Bits64(flags, 23, 8) == 0
        && Bits64(flags, 7, 0) == ArrayLength(possible_parents)
        && Bits64(flags, 31, 24) == RemainingPossibleParents(old_s, clock_id, skip_parents, Bits64(flags, 7, 0))
        && (forall|i: UInt32| i < Bits64(flags, 7, 0) ==> ElementAt(possible_parents, i) == NthPossibleParent(old_s, clock_id, skip_parents + i))
        && (forall|i: UInt32| i + 1 < Bits64(flags, 7, 0) ==> ElementAt(possible_parents, i) < ElementAt(possible_parents, i + 1))
    ))
}