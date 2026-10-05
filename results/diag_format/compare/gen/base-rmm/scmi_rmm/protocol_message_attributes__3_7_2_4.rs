pub open spec fn protocol_message_attributes__3_7_2_4_spec(result: Int32, attributes: UInt32, old_s: S, new_s: S) -> bool {
    (!IsMessageImplemented(old_s, message_id) ==> ResultEqual(result, NOT_FOUND))
    && (ResultEqual(result, SUCCESS) ==> attributes == 0)
    && (old_s == new_s)
}