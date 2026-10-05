pub open spec fn protocol_message_attributes__3_6_2_4_spec(message_id: UInt32, status: Int32, attributes: UInt32, old_s: S, new_s: S) -> bool {
    (!IsClockProtocolMessageImplemented(old_s, message_id) ==> status == NOT_FOUND)
    && (IsClockProtocolMessageImplemented(old_s, message_id) ==> (status == SUCCESS && attributes == 0))
    && (new_s == old_s)
}
