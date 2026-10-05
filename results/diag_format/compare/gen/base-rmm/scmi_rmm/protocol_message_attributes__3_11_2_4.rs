pub open spec fn protocol_message_attributes__3_11_2_4_spec(status: Int32, attributes: UInt32, message_id: UInt32, old_s: S, new_s: S) -> bool {
    (!IsValidMessageId(message_id) ==> ResultEqual(status, NOT_FOUND))
    && (!IsMessageImplemented(message_id) ==> ResultEqual(status, NOT_FOUND))
    && (!IsMessageAvailableToAgent(message_id) ==> ResultEqual(status, NOT_FOUND))
    && (ResultEqual(status, SUCCESS) ==> attributes == 0)
    && (old_s == new_s)
}