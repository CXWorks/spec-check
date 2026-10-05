pub open spec fn sbi_mpxy_send_message_with_response_spec(channel_id: uint32_t, message_id: uint32_t, message_data_len: unsigned long, result: SbiCommandReturnCode, uvalue: UInt64, old_s: S, new_s: S) -> bool {
  (result == SBI_SUCCESS ==> MessageResponseReceived(new_s, channel_id))
  && (result == SBI_SUCCESS ==> ResultEqual(result, SBI_SUCCESS))
  && (result == SBI_SUCCESS ==> SharedMemoryAt(new_s, calling_hart(new_s), 0x0) == MessageResponseData(new_s, channel_id))
  && (result == SBI_SUCCESS ==> uvalue == MessageResponseDataLength(new_s, channel_id))
  && ((!(result == SBI_SUCCESS))
    ==> SharedMemoryAt(new_s, calling_hart(new_s), 0x0) == SharedMemoryAt(old_s, calling_hart(old_s), 0x0))
}