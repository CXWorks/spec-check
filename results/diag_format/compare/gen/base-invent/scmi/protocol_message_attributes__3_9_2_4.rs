pub open spec fn protocol_message_attributes__3_9_2_4_spec(result: int32, attributes: uint32, old_s: S, new_s: S) -> bool {
    (result == NOT_FOUND ==> message_id_invalid_or_not_implemented(old_s, message_id))
    && (result == SUCCESS ==> message_id_valid_and_implemented(old_s, message_id))
    && (result == SUCCESS ==> attributes == 0)
    && (result == SUCCESS ==> old_s == new_s)
}

fn message_id_invalid_or_not_implemented(old_s: S, message_id: uint32) -> bool {
    true
}

fn message_id_valid_and_implemented(old_s: S, message_id: uint32) -> bool {
    true
}