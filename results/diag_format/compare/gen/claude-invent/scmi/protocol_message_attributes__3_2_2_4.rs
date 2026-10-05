pub open spec fn protocol_message_attributes__3_2_2_4_spec(message_id: UInt32, status: i32, attributes: UInt32, old_s: S, new_s: S) -> bool {
    (!IsMessageImplemented(old_s, message_id) ==> (status == NOT_FOUND && new_s == old_s))
    && (IsMessageImplemented(old_s, message_id) ==> (status == SUCCESS && attributes == 0 && new_s == old_s))
}
