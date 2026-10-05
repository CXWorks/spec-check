pub open spec fn clock_parent_get__3_6_2_16_spec(result: int32, parent_id: uint32, old_s: S, new_s: S) -> bool {
    (!ClockExists(old_s, clock_id) ==> ResultEqual(result, NOT_FOUND))
    && (!IsRequestSupported(old_s, clock_id) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!AgentMayGetClockParent(old_s, calling_agent, clock_id) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> parent_id == ClockParent(old_s, clock_id))
    && (ResultEqual(result, SUCCESS) ==> old_s == new_s)
}