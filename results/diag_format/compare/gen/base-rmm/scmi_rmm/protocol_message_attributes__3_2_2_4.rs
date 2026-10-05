pub open spec fn protocol_message_attributes__3_2_2_4_spec(status: int32, attributes: uint32, message_id: uint32, old_s: S, new_s: S) -> bool {
    (!IsMessageImplemented(message_id) ==> ResultEqual(status, NOT_FOUND))
    && (ResultEqual(status, SUCCESS) ==> attributes == 0)
    && (old_s == new_s)
}