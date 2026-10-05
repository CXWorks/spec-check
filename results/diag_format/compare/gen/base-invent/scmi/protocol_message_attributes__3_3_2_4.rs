pub open spec fn protocol_message_attributes_spec(result: int32, attributes: uint32, old_s: S, new_s: S) -> bool {
    (result == SUCCESS ==> attributes == 0)
    && (result == NOT_FOUND ==> true)
}