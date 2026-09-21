pub open spec fn sbi_mpxy_send_message_with_response_spec(channel_id: UInt32, message_id: UInt32, message_data_len: UInt64, result: sbiret, old_s: S, new_s: S) -> bool {
  (result.error == SBI_SUCCESS)
}