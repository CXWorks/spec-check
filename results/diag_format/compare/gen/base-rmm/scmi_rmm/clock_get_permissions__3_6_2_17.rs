pub open spec fn clock_get_permissions__3_6_2_17_spec(result: int32, permissions: uint32, old_s: S, new_s: S) -> bool {
    (!IsValidClockId(clock_id) ==> ResultEqual(result, NOT_FOUND))
    && (!IsRequestSupported(CLOCK_GET_PERMISSIONS) ==> ResultEqual(result, NOT_SUPPORTED))
    && (ResultEqual(result, SUCCESS) ==> (
        permissions[31] == (AgentCanChangeClockState(clock_id) ? 1 : 0)
        && (permissions[31] == 0 ==> CLOCK_CONFIG_SET returns DENIED for attempts to change the state of clock_id)
        && permissions[30] == (AgentCanChangeClockParent(clock_id) ? 1 : 0)
        && (permissions[30] == 0 ==> CLOCK_PARENT_SET returns DENIED for clock_id)
        && permissions[29] == (AgentCanChangeClockRate(clock_id) ? 1 : 0)
        && (permissions[29] == 0 ==> CLOCK_RATE_SET returns DENIED for clock_id)
        && permissions[28:0] == 0
    ))
}