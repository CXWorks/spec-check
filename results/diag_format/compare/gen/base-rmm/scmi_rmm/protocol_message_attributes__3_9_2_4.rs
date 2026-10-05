pub open spec fn protocol_message_attributes__3_9_2_4_spec(result: Int32, attributes: UInt32, old_s: S, new_s: S) -> bool {
    (!IsValidMessageId(message_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (!IsMessageImplemented(message_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (ResultEqual(result, NOT_FOUND) ==> !IsMessageImplemented(message_id(old_s)) || !IsMessageAvailable(message_id(old_s)))
    && (ResultEqual(result, SUCCESS) ==> IsMessageImplemented(message_id(old_s)) && IsMessageAvailable(message_id(old_s)))
    && (ResultEqual(result, SUCCESS) ==> attributes == 0)
    && (old_s == new_s)
}