pub open spec fn sbi_mpxy_send_message_without_response_spec(channel_id: UInt32, message_id: UInt32, message_data_len: UInt, result: SbiErrorCode, old_s: S, new_s: S) -> bool {
  (message_data_len > MsgDataMaxLen(old_s, channel_id) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (message_data_len > SharedMemorySize(old_s, CallingHart()) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
  && (result == SBI_SUCCESS ==> ResultEqual(result, SBI_SUCCESS))
  && (result == SBI_SUCCESS ==> MessageTransmitted(new_s, channel_id, message_id, SharedMemoryData(new_s, CallingHart(), 0, message_data_len)))
  && ((! (message_data_len > MsgDataMaxLen(old_s, channel_id)) &&
       ! (message_data_len > SharedMemorySize(old_s, CallingHart())))
    ==> result == SBI_SUCCESS)
  && (result != SBI_SUCCESS
    ==> !MessageTransmitted(new_s, channel_id, message_id, SharedMemoryData(new_s, CallingHart(), 0, message_data_len)))
}