pub open spec fn protocol_message_attributes__3_2_2_4_spec(result: Int32, attributes: UInt32, old_s: S, new_s: S) -> bool {
    (!IsMessageImplemented(0x10, message_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (IsMessageImplemented(0x10, message_id(old_s)) ==> (ResultEqual(result, SUCCESS) && IsMessageAvailable(0x10, message_id(old_s)) && attributes == 0))
    && (old_s == new_s)
}