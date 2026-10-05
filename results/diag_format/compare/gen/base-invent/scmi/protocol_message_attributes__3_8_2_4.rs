pub open spec fn protocol_message_attributes__3_8_2_4_spec(result: int32, attributes: uint32, old_s: S, new_s: S) -> bool {
    (result == NOT_FOUND ==> attributes == 0)
    && (result == SUCCESS ==> attributes == 0)
}