pub open spec fn sbi_mpxy_send_message_without_response_spec(channel_id: UInt32, message_id: UInt32, message_data_len: UInt64, result: sbiret, old_s: S, new_s: S) -> bool {
  (result.error == SBI_ERR_INVALID_PARAM ==> message_data_len > MSG_DATA_MAX_LEN)
  && (result.error == SBI_ERR_INVALID_PARAM ==> message_data_len > shared_memory_size(old_s))
  && ((!(result.error == SBI_ERR_INVALID_PARAM) &&
       result.error == SBI_SUCCESS)
    ==> true)
  && (result.error != SBI_SUCCESS && result.error != SBI_ERR_INVALID_PARAM
    ==> true)
}