pub open spec fn protocol_message_attributes__3_6_2_4_spec(
    result: Int32,
    message_id: UInt32,
    calling_agent: UInt32,
    old_s: S,
    new_s: S,
) -> bool {
    // Failure conditions
    (!IsValidMessageId(message_id) ==> ResultEqual(result, NOT_FOUND))
    && (!IsMessageImplemented(message_id) ==> ResultEqual(result, NOT_FOUND))
    && ((message_id == CLOCK_RATE_NOTIFY || message_id == CLOCK_RATE_CHANGE_REQUESTED_NOTIFY)
        && !IsNotificationAvailableToAgent(message_id, calling_agent)
        ==> ResultEqual(result, NOT_FOUND))
    // Success conditions
    && (ResultEqual(result, SUCCESS)
        ==> (IsMessageImplemented(message_id)
            && IsMessageAvailableToAgent(message_id, calling_agent)
            && attributes == 0
            && (message_id == CLOCK_RATE_NOTIFY || message_id == CLOCK_RATE_CHANGE_REQUESTED_NOTIFY)
                ==> IsNotificationSupported(message_id)))
    // Unchanged state
    && (old_s == new_s)
}