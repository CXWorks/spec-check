pub open spec fn 3.11.2.4_protocol_message_attributes_spec(message_id: UInt32, result: RsiCommandReturnCode, attributes: UInt32, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> attributes == 0)
}