pub open spec fn protocol_message_attributes__3_5_6_4_spec(message_id: UInt32, status: Int32, attributes: UInt32, old_s: S, new_s: S) -> bool {
  (!IsValidMessageId(old_s, message_id) || !IsMessageImplemented(old_s, message_id) ==> ResultEqual(status, NOT_FOUND))
  && ((message_id == PERFORMANCE_NOTIFY_LEVEL || message_id == PERFORMANCE_NOTIFY_LIMITS) && (!IsNotificationImplemented(old_s, message_id) || !IsNotificationAvailableToAgent(old_s, message_id, calling_agent)) ==> ResultEqual(status, NOT_FOUND))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> Bits(attributes, 31, 1) == 0)
  && (ResultEqual(status, SUCCESS) ==> HasDedicatedFastChannel(old_s, message_id) ==> Bits(attributes, 0, 0) == 1)
  && (ResultEqual(status, SUCCESS) ==> !HasDedicatedFastChannel(old_s, message_id) ==> Bits(attributes, 0, 0) == 0)
  && ((!(IsValidMessageId(old_s, message_id) || !IsMessageImplemented(old_s, message_id)) &&
       !(((message_id == PERFORMANCE_NOTIFY_LEVEL || message_id == PERFORMANCE_NOTIFY_LIMITS) && (!IsNotificationImplemented(old_s, message_id) || !IsNotificationAvailableToAgent(old_s, message_id, calling_agent))))
    ==> ResultEqual(status, SUCCESS))
}