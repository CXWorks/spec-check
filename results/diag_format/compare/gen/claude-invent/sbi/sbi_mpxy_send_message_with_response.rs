pub open spec fn sbi_mpxy_send_message_with_response_spec(channel_id: UInt32, message_id: UInt32, message_data_len: UInt64, result: SbiRet, old_s: S, new_s: S) -> bool {
    ((result.error == SBI_SUCCESS) ==> (MpxyResponseReceived(old_s, channel_id, message_id, message_data_len) && MpxyChannelCapabilitySendWithResponse(old_s, channel_id)))
    && ((MpxyChannelCapabilitySendWithResponse(old_s, channel_id) && MpxyResponseReceived(old_s, channel_id, message_id, message_data_len)) ==> (result.error == SBI_SUCCESS && result.uvalue == MpxyResponseDataLen(old_s, channel_id, message_id, message_data_len) && MpxyShmemResponseWritten(old_s, new_s, channel_id, message_id, message_data_len, result.uvalue)))
}
