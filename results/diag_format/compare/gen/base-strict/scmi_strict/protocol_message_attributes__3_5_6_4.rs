pub open spec fn protocol_message_attributes__3_5_6_4_spec(result: Int32, attributes: UInt32, old_s: S, new_s: S) -> bool {
    (!IsValidMessageId(message_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (!IsMessageImplemented(message_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && ((message_id(old_s) == PERFORMANCE_NOTIFY_LEVEL || message_id(old_s) == PERFORMANCE_NOTIFY_LIMITS) && (!IsNotificationImplemented(message_id(old_s)) || !IsNotificationAvailableToAgent(message_id(old_s), calling_agent(old_s))) ==> ResultEqual(result, NOT_FOUND))
    && (ResultEqual(result, SUCCESS) ==> Bits(attributes, 31, 1) == 0)
    && (HasDedicatedFastChannel(message_id(old_s)) ==> Bits(attributes, 0, 0) == 1)
    && (!HasDedicatedFastChannel(message_id(old_s)) ==> Bits(attributes, 0, 0) == 0)
    && (old_s == new_s)
}