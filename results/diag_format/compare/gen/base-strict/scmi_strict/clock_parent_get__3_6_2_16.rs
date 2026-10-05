pub open spec fn clock_parent_get__3_6_2_16_spec(result: Int32, parent_id: UInt32, old_s: S, new_s: S) -> bool {
    (!ClockExists(old_s, clock_id) ==> ResultEqual(result, NOT_FOUND))
    && (!IsClockParentGetSupported(old_s, clock_id) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsAgentAllowedToGetClockParent(CallingAgent(), clock_id) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> parent_id == ClockParent(clock_id))
}