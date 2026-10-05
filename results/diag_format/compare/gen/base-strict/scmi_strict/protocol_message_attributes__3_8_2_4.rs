pub open spec fn protocol_message_attributes__3_8_2_4_spec(result: int32, attributes: uint32, old_s: S, new_s: S) -> bool {
    (!IsMessageProvided(old_s, message_id) ==> ResultEqual(result, NOT_FOUND))
    && (ResultEqual(result, SUCCESS) ==> (IsMessageImplementedAndAvailable(old_s, message_id) && attributes == 0))
    && (old_s == new_s)
}