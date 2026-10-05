pub open spec fn protocol_message_attributes__3_4_2_4_spec(message_id: UInt32, status: Int32, attributes: UInt32, old_s: S, new_s: S) -> bool {
  (!IsMessageImplemented(old_s, message_id) ==> ResultEqual(status, NOT_FOUND))
  && (message_id == SYSTEM_POWER_STATE_NOTIFY_ID && !SystemPowerStateNotificationsSupported(old_s) ==> ResultEqual(status, NOT_SUPPORTED))
  && (ResultEqual(status, SUCCESS))
  && (message_id == SYSTEM_POWER_STATE_SET_ID ==> (Bits(attributes, 31, 31) == 1) == SystemWarmResetSupported(old_s))
  && (message_id == SYSTEM_POWER_STATE_SET_ID ==> (Bits(attributes, 30, 30) == 1) == SystemSuspendSupported(old_s))
  && (message_id == SYSTEM_POWER_STATE_SET_ID ==> Bits(attributes, 29, 0) == 0)
  && (message_id != SYSTEM_POWER_STATE_SET_ID ==> attributes == 0)
  && ((!(IsMessageImplemented(old_s, message_id)) &&
       !(message_id == SYSTEM_POWER_STATE_NOTIFY_ID && !SystemPowerStateNotificationsSupported(old_s)))
    ==> ResultEqual(status, SUCCESS))
}