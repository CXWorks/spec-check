pub open spec fn protocol_message_attributes__3_9_2_4_spec(message_id: u32, status: i32, attributes: u32, old_s: S, new_s: S) -> bool {
    (!ScmiMessageImplemented(old_s, 0x17u32, message_id) ==> status == NOT_FOUND)
    && (ScmiMessageImplemented(old_s, 0x17u32, message_id) ==> (status == SUCCESS && attributes == 0u32))
    && (status == SUCCESS ==> attributes == 0u32)
    && new_s == old_s
}
