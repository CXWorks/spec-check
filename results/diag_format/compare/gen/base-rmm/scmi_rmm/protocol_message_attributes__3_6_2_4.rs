pub open spec fn protocol_message_attributes_spec(result: Int32, attributes: UInt32, old_s: S, new_s: S) -> bool {
    (!IsMessageImplemented(old_s, header.message_id) ==> ResultEqual(result, NOT_FOUND))
    && (!IsMessageAvailableToAgent(old_s, header.message_id) ==> ResultEqual(result, NOT_FOUND))
    && (ResultEqual(result, SUCCESS) ==> attributes == 0)
    && (ResultEqual(result, SUCCESS) ==> old_s == new_s)
}