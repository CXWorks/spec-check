pub open spec fn sbi_mpxy_send_message_with_response_spec(error: sbiret, uvalue: u64, old_s: S, new_s: S) -> bool {
    (MessageResponseReceived(channel_id(old_s)) ==> ResultEqual(error, SBI_SUCCESS))
    && (ResultEqual(error, SBI_SUCCESS) ==> MessageResponseReceived(channel_id(old_s)))
    && (ResultEqual(error, SBI_SUCCESS) ==> SharedMemoryAt(calling_hart(old_s), 0x0) == MessageResponseData(channel_id(old_s)))
    && (ResultEqual(error, SBI_SUCCESS) ==> uvalue == MessageResponseDataLength(channel_id(old_s)))
}