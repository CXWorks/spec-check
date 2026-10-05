pub open spec fn sbi_mpxy_send_message_without_response_spec(error: SbiErrorCode, old_s: S, new_s: S) -> bool {
    (old_s.message_data_len > MsgDataMaxLen(old_s.channel_id) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
    && (old_s.message_data_len > SharedMemorySize(old_s.calling_hart) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
    && (ResultEqual(error, SBI_SUCCESS) ==> MessageTransmitted(old_s.channel_id, old_s.message_id, SharedMemory(old_s.calling_hart)[0x0 : old_s.message_data_len]))
}