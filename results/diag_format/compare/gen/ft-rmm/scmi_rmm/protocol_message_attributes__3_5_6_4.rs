pub open spec fn protocol_message_attributes__3_5_6_4_spec(message_id: UInt32, status: Int32, attributes: UInt32, old_s: S, new_s: S) -> bool {
  (!IsValidMessage(old_s, message_id) || !IsImplementedMessage(old_s, message_id) ==> ResultEqual(status, NOT_FOUND))
  && (IsPerformanceNotifyMessage(old_s, message_id) && (!NotificationsImplemented(old_s, message_id) || !NotificationsAvailableToAgent(old_s, message_id, calling_agent)) ==> ResultEqual(status, NOT_FOUND))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> attributes[31:1] == 0)
  && (ResultEqual(status, SUCCESS) ==> attributes[0] == (HasDedicatedFastChannel(old_s, message_id) ? 1 : 0))
  && ((!(IsValidMessage(old_s, message_id)) && IsImplementedMessage(old_s, message_id))
    ==> ResultEqual(status, SUCCESS))
  && (!(IsPerformanceNotifyMessage(old_s, message_id) && (!NotificationsImplemented(old_s, message_id) || !NotificationsAvailableToAgent(old_s, message_id, calling_agent)))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> attributes[31:1] == 0)
  && (result != SUCCESS
    ==> attributes[0] == (HasDedicatedFastChannel(old_s, message_id) ? 1 : 0))
}