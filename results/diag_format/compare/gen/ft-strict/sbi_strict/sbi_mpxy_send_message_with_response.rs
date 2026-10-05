pub open spec fn sbi_mpxy_send_message_with_response_spec(channel_id: UInt32, message_id: UInt32, message_data_len: UInt, error: SbiErrorCode, uvalue: UInt, old_s: S, new_s: S) -> bool {
  (MessageResponseReceived(new_s, channel_id, message_id))
  && (SharedMemResponseWritten(new_s, CallingHart(new_s), 0, uvalue))
  && (uvalue == MessageResponseDataLen(new_s, channel_id, message_id))
  && (ChannelCapabilitySendWithResponseBit(new_s, channel_id) == 1)
  && (ResultEqual(error, SBI_SUCCESS))
  && ((!(MessageResponseReceived(old_s, channel_id, message_id)) &&
       !(SharedMemResponseWritten(old_s, CallingHart(old_s), 0, uvalue)) &&
       !(uvalue == MessageResponseDataLen(old_s, channel_id, message_id)) &&
       !(ChannelCapabilitySendWithResponseBit(old_s, channel_id) == 1) &&
       !(ResultEqual(error, SBI_SUCCESS)))
    ==> (MessageResponseReceived(new_s, channel_id, message_id) &&
         SharedMemResponseWritten(new_s, CallingHart(new_s), 0, uvalue) &&
         uvalue == MessageResponseDataLen(new_s, channel_id, message_id) &&
         ChannelCapabilitySendWithResponseBit(new_s, channel_id) == 1 &&
         ResultEqual(error, SBI_SUCCESS)))
}