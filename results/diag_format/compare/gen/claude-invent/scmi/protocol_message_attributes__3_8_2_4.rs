pub open spec fn protocol_message_attributes__3_8_2_4_spec(message_id: u32, status: i32, attributes: u32, old_s: S, new_s: S) -> bool {
    (!IsMessageImplemented(old_s, message_id) ==> (status == NOT_FOUND))
    && (IsMessageImplemented(old_s, message_id) ==> (status == SUCCESS && attributes == 0))
    && (new_s == old_s)
}
