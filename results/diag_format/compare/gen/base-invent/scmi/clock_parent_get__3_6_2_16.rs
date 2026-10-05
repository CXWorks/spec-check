pub open spec fn clock_parent_get__3_6_2_16_spec(result: int32, parent_id: uint32, old_s: S, new_s: S) -> bool {
    (result == NOT_FOUND ==> !ClockExists(old_s, clock_id))
    && (result == NOT_SUPPORTED ==> !ClockSupported(old_s, clock_id))
    && (result == DENIED ==> !AgentAllowedToGetParent(old_s, clock_id))
    && (result == SUCCESS ==> ClockExists(old_s, clock_id) && ClockSupported(old_s, clock_id) && AgentAllowedToGetParent(old_s, clock_id) && parent_id == ClockParent(old_s, clock_id))
}