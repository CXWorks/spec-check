pub open spec fn protocol_message_attributes__3_7_2_4_spec(result: int32, attributes: uint32, old_s: S, new_s: S) -> bool {
    (result == NOT_FOUND ==> message_id_not_supported(old_s, message_id))
    && (result == SUCCESS ==> (attributes == 0u32))
    && (result != SUCCESS && result != NOT_FOUND ==> true)
    && (result == SUCCESS ==> true)
}

fn message_id_not_supported(old_s: S, message_id: uint32) -> bool {
    true
}