pub open spec fn protocol_message_attributes__3_12_4_4_spec(status: i32, attributes: UInt32, message_id: UInt32, old_s: S, new_s: S) -> bool {
    (!IsMessageImplementedAndAvailable(old_s, 0x1B, message_id) ==> status == NOT_FOUND)
    && (IsMessageImplementedAndAvailable(old_s, 0x1B, message_id) ==> (status == SUCCESS && attributes == 0))
    && (new_s == old_s)
}
