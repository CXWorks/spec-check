pub open spec fn sbi_mpxy_send_message_without_response_spec(channel_id: UInt32, message_id: UInt32, message_data_len: UInt, error: SbiErrorCode, old_s: S, new_s: S) -> bool {
  (message_data_len > MsgDataMaxLen(old_s, channel_id) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
  && (message_data_len > SharedMemorySize(old_s, calling_hart) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
  && (ResultEqual(error, SBI_SUCCESS) ==> ResultEqual(error, SBI_SUCCESS))
  && (ResultEqual(error, SBI_SUCCESS) ==> MessageTransmitted(new_s, channel_id, message_id, SharedMemory(new_s, calling_hart)[0x0 : message_data_len]))
  && ((! (message_data_len > MsgDataMaxLen(old_s, channel_id)) &&
       ! (message_data_len > SharedMemorySize(old_s, calling_hart)))
    ==> ResultEqual(error, SBI_SUCCESS))
  && (ResultNotEqual(error, SBI_SUCCESS)
    ==> !MessageTransmitted(new_s, channel_id, message_id, SharedMemory(new_s, calling_hart)[0x0 : message_data_len]))
  && (MpxyChannel(new_s, channel_id).transmitted_messages == MpxyChannel(old_s, channel_id).transmitted_messages + 1)
  && (ResultNotEqual(error, SBI_SUCCESS)
    ==> MpxyChannel(new_s, channel_id).transmitted_messages == MpxyChannel(old_s, channel_id).transmitted_messages)
}