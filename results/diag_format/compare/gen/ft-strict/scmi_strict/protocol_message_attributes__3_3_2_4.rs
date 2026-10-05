pub open spec fn protocol_message_attributes__3_3_2_4_spec(message_id: UInt32, status: Int32, attributes: UInt32, old_s: S, new_s: S) -> bool {
  (!IsValidMessageId(old_s, message_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsMessageImplemented(old_s, message_id) ==> ResultEqual(status, NOT_FOUND))
  && (ResultEqual(status, SUCCESS) ==> IsMessageImplemented(new_s, message_id) && IsMessageAvailableToCaller(new_s, message_id))
  && (ResultEqual(status, SUCCESS) ==> attributes == 0)
  && (ResultEqual(status, SUCCESS) && (message_id == POWER_STATE_NOTIFY || message_id == POWER_STATE_CHANGE_REQUESTED_NOTIFY) ==> PowerStateNotificationsSupported(new_s, message_id))
  && ((IsValidMessageId(old_s, message_id) &&
       IsMessageImplemented(old_s, message_id))
    ==> ResultEqual(status, SUCCESS))
}