pub open spec fn protocol_message_attributes_spec(result: Int32, attributes: UInt32, message_id: UInt32, old_s: S, new_s: S) -> bool {
    (!IsValidMessageId(message_id) ==> ResultEqual(result, NOT_FOUND))
    && (!IsMessageImplemented(message_id) ==> ResultEqual(result, NOT_FOUND))
    && (IsPowerStateNotifyMessage(message_id) && !IsNotificationAvailableToAgent(message_id) ==> ResultEqual(result, NOT_FOUND))
    && (ResultEqual(result, SUCCESS) ==> attributes == 0)
    && (old_s == new_s)
}