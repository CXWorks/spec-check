pub open spec fn protocol_message_attributes__3_4_2_4_spec(result: Int32, attributes: UInt32, message_id: UInt32, old_s: S, new_s: S) -> bool {
    (!IsMessageImplemented(message_id) ==> ResultEqual(result, NOT_FOUND))
    && (message_id == SYSTEM_POWER_STATE_NOTIFY_ID && !SystemPowerStateNotificationsSupported() ==> ResultEqual(result, NOT_SUPPORTED))
    && ResultEqual(result, SUCCESS)
    && (message_id == SYSTEM_POWER_STATE_SET_ID ==> ((attributes >> 31) as int == 1) == SystemWarmResetSupported())
    && (message_id == SYSTEM_POWER_STATE_SET_ID ==> ((attributes >> 30) as int == 1) == SystemSuspendSupported())
    && (message_id == SYSTEM_POWER_STATE_SET_ID ==> (attributes & 0x1FFFFFFF) == 0)
    && (message_id != SYSTEM_POWER_STATE_SET_ID ==> attributes == 0)
    && old_s == new_s
}