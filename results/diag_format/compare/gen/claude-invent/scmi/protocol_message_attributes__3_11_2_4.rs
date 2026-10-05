pub open spec fn protocol_message_attributes__3_11_2_4_spec(message_id: UInt32, status: i32, attributes: UInt32, old_s: S, new_s: S) -> bool {
    (!IsScmiMessageImplemented(old_s, 0x19, message_id) ==> status == NOT_FOUND)
    && (IsScmiMessageImplemented(old_s, 0x19, message_id) ==> (status == SUCCESS && attributes == 0))
    && (status == SUCCESS ==> attributes == 0)
    && new_s == old_s
}
