pub open spec fn protocol_message_attributes__3_4_2_4_spec(message_id: UInt32, status: Int32, attributes: UInt32, old_s: S, new_s: S) -> bool {
  (!IsImplementedMessage(old_s, message_id) ==> ResultEqual(status, NOT_FOUND))
  && (message_id == SYSTEM_POWER_STATE_NOTIFY_ID && !PowerStateNotificationsSupported(old_s) ==> ResultEqual(status, NOT_SUPPORTED))
  && (ResultEqual(status, SUCCESS) ==> message_id == SYSTEM_POWER_STATE_SET_ID ==> attributes[31] == (SystemWarmResetSupported(old_s) ? 1 : 0))
  && (ResultEqual(status, SUCCESS) ==> message_id == SYSTEM_POWER_STATE_SET_ID ==> attributes[30] == (SystemSuspendSupported(old_s) ? 1 : 0))
  && (ResultEqual(status, SUCCESS) ==> message_id == SYSTEM_POWER_STATE_SET_ID ==> attributes[29:0] == 0)
  && (ResultEqual(status, SUCCESS) ==> message_id != SYSTEM_POWER_STATE_SET_ID ==> attributes == 0)
  && ((!(IsImplementedMessage(old_s, message_id)) &&
       !(message_id == SYSTEM_POWER_STATE_NOTIFY_ID && !PowerStateNotificationsSupported(old_s)))
    ==> ResultEqual(status, SUCCESS))
}