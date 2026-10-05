pub open spec fn protocol_message_attributes__3_7_2_4_spec(message_id: UInt32, status: i32, attributes: UInt32, old_s: S, new_s: S) -> bool {
    (!IsScmiMessageImplemented(old_s, 0x15u32, message_id) ==> status == NOT_FOUND)
    && (IsScmiMessageImplemented(old_s, 0x15u32, message_id) ==> status != NOT_FOUND)
    && (status == SUCCESS ==> (IsScmiMessageImplemented(old_s, 0x15u32, message_id) && attributes == 0))
    && (new_s == old_s)
}
