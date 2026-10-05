pub open spec fn clock_get_permissions__3_6_2_17_spec(clock_id: u32, status: i32, permissions: u32, old_s: S, new_s: S) -> bool {
    (!ClockGetPermissionsSupported(old_s) ==> status == NOT_SUPPORTED)
    && (ClockGetPermissionsSupported(old_s) && !ClockIsValid(old_s, clock_id) ==> status == NOT_FOUND)
    && (ClockGetPermissionsSupported(old_s) && ClockIsValid(old_s, clock_id) ==> (
        status == SUCCESS
        && (((permissions & 0x8000_0000u32) != 0u32) == AgentCanChangeClockState(old_s, clock_id))
        && (((permissions & 0x4000_0000u32) != 0u32) == AgentCanChangeClockParent(old_s, clock_id))
        && (((permissions & 0x2000_0000u32) != 0u32) == AgentCanChangeClockRate(old_s, clock_id))
        && ((permissions & 0x1FFF_FFFFu32) == 0u32)
    ))
    && (new_s == old_s)
}
