pub open spec fn protocol_message_attributes__3_3_2_4_spec(message_id: UInt32, status: Int32, attributes: UInt32, old_s: S, new_s: S) -> bool {
  (!IsValidMessageId(old_s, message_id) || !IsMessageImplemented(old_s, message_id) ==> ResultEqual(status, NOT_FOUND))
  && (IsPowerStateNotifyMessage(old_s, message_id) && !IsNotificationAvailableToAgent(old_s, message_id) ==> ResultEqual(status, NOT_FOUND))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> attributes == 0)
  && ((IsValidMessageId(old_s, message_id) &&
       IsMessageImplemented(old_s, message_id) &&
       !(IsPowerStateNotifyMessage(old_s, message_id) && !IsNotificationAvailableToAgent(old_s, message_id)))
    ==> ResultEqual(status, SUCCESS))
}