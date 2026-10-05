pub open spec fn protocol_message_attributes__3_4_2_4_spec(message_id: UInt32, status: i32, attributes: UInt32, old_s: S, new_s: S) -> bool {
    (!IsMessageImplemented(old_s, message_id) ==> (status == NOT_FOUND && new_s == old_s))
    && ((IsMessageImplemented(old_s, message_id)
        && message_id == SYSTEM_POWER_STATE_NOTIFY
        && !SystemPowerStateNotificationsSupported(old_s)) ==> (status == NOT_SUPPORTED && new_s == old_s))
    && ((IsMessageImplemented(old_s, message_id)
        && !(message_id == SYSTEM_POWER_STATE_NOTIFY && !SystemPowerStateNotificationsSupported(old_s))) ==> (
        status == SUCCESS
        && (message_id == SYSTEM_POWER_STATE_SET ==> (
            (((attributes & 0x8000_0000u32) != 0) == SystemWarmResetSupported(old_s))
            && (((attributes & 0x4000_0000u32) != 0) == SystemSuspendSupported(old_s))
            && ((attributes & 0x3FFF_FFFFu32) == 0)
        ))
        && (message_id != SYSTEM_POWER_STATE_SET ==> attributes == 0)
        && new_s == old_s
    ))
}
