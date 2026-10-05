pub open spec fn protocol_message_attributes__3_5_6_4_spec(result: Int32, attributes: UInt32, old_s: S, new_s: S) -> bool {
    (!IsValidMessage(old_s, message_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (!IsImplementedMessage(old_s, message_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (IsPerformanceNotifyMessage(old_s, message_id(old_s)) && (!NotificationsImplemented(old_s, message_id(old_s)) || !NotificationsAvailableToAgent(old_s, message_id(old_s), calling_agent(old_s))) ==> ResultEqual(result, NOT_FOUND))
    && (ResultEqual(result, SUCCESS) ==> attributes[31:1] == 0)
    && (ResultEqual(result, SUCCESS) ==> attributes[0] == (HasDedicatedFastChannel(old_s, message_id(old_s)) ? 1 : 0))
}