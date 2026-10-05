pub open spec fn protocol_message_attributes__3_4_2_4_spec(result: Int32, attributes: UInt32, old_s: S, new_s: S) -> bool {
    (!IsImplementedMessage(message_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (message_id(old_s) == SYSTEM_POWER_STATE_NOTIFY_ID && !PowerStateNotificationsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
    && ResultEqual(result, SUCCESS)
    && (message_id(old_s) == SYSTEM_POWER_STATE_SET_ID ==> (attributes[31] == (SystemWarmResetSupported(old_s) ? 1 : 0)))
    && (message_id(old_s) == SYSTEM_POWER_STATE_SET_ID ==> (attributes[30] == (SystemSuspendSupported(old_s) ? 1 : 0)))
    && (message_id(old_s) == SYSTEM_POWER_STATE_SET_ID ==> (attributes[29:0] == 0))
    && (message_id(old_s) != SYSTEM_POWER_STATE_SET_ID ==> attributes == 0)
    && (old_s == new_s)
}