pub open spec fn clock_parent_get__3_6_2_16_spec(clock_id: u32, status: i32, parent_id: u32, old_s: S, new_s: S) -> bool {
    (!ClockExists(old_s, clock_id) ==> status == NOT_FOUND)
    && (status == NOT_FOUND ==> !ClockExists(old_s, clock_id))
    && (status == NOT_SUPPORTED ==> !ClockParentGetSupported(old_s, clock_id))
    && (status == DENIED ==> !AgentAllowedClockParentGet(old_s, clock_id))
    && (status == SUCCESS ==> (
        ClockExists(old_s, clock_id)
        && ClockParentGetSupported(old_s, clock_id)
        && AgentAllowedClockParentGet(old_s, clock_id)
        && parent_id == ClockParentOf(old_s, clock_id)
    ))
    && (new_s == old_s)
}
