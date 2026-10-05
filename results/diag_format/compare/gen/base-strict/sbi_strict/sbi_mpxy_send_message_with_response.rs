pub open spec fn sbi_mpxy_send_message_with_response_spec(error: SbiErrorCode, uvalue: UInt, old_s: S, new_s: S) -> bool {
    (MessageResponseReceived(old_s, channel_id(old_s), message_id(old_s)) ==> ResultEqual(error, SBI_SUCCESS))
    && (SharedMemResponseWritten(old_s, CallingHart(old_s), 0, uvalue) ==> ResultEqual(error, SBI_SUCCESS))
    && (uvalue == MessageResponseDataLen(old_s, channel_id(old_s), message_id(old_s)) ==> ResultEqual(error, SBI_SUCCESS))
    && (ChannelCapabilitySendWithResponseBit(old_s, channel_id(old_s)) == 1 ==> ResultEqual(error, SBI_SUCCESS))
    && (ResultEqual(error, SBI_SUCCESS) ==> MessageResponseReceived(old_s, channel_id(old_s), message_id(old_s)))
    && (ResultEqual(error, SBI_SUCCESS) ==> SharedMemResponseWritten(old_s, CallingHart(old_s), 0, uvalue))
    && (ResultEqual(error, SBI_SUCCESS) ==> uvalue == MessageResponseDataLen(old_s, channel_id(old_s), message_id(old_s)))
    && (ResultEqual(error, SBI_SUCCESS) ==> ChannelCapabilitySendWithResponseBit(old_s, channel_id(old_s)) == 1)
}