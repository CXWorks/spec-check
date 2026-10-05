pub open spec fn protocol_message_attributes__3_6_2_4_spec(message_id: UInt32, status: Int32, attributes: UInt32, calling_agent: CallingAgent, old_s: S, new_s: S) -> bool {
  (!IsValidMessageId(old_s, message_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsMessageImplemented(old_s, message_id) ==> ResultEqual(status, NOT_FOUND))
  && ((message_id == CLOCK_RATE_NOTIFY || message_id == CLOCK_RATE_CHANGE_REQUESTED_NOTIFY) && !IsNotificationAvailableToAgent(old_s, message_id, calling_agent) ==> ResultEqual(status, NOT_FOUND))
  && (ResultEqual(status, SUCCESS) ==> IsMessageImplemented(new_s, message_id) && IsMessageAvailableToAgent(new_s, message_id, calling_agent))
  && (ResultEqual(status, SUCCESS) ==> attributes == 0)
  && (ResultEqual(status, SUCCESS) && (message_id == CLOCK_RATE_NOTIFY || message_id == CLOCK_RATE_CHANGE_REQUESTED_NOTIFY) ==> IsNotificationSupported(new_s, message_id))
  && ((IsValidMessageId(old_s, message_id) &&
       IsMessageImplemented(old_s, message_id) &&
       !((message_id == CLOCK_RATE_NOTIFY || message_id == CLOCK_RATE_CHANGE_REQUESTED_NOTIFY) && !IsNotificationAvailableToAgent(old_s, message_id, calling_agent)))
    ==> ResultEqual(status, SUCCESS))
}