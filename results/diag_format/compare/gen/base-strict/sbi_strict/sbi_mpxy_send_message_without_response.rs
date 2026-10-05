pub open spec fn sbi_mpxy_send_message_without_response_spec(result: SbiErrorCode, old_s: S, new_s: S) -> bool {
    (MessageDataMaxLen(old_s, ChannelId(old_s)) < MessageDataLen(old_s) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (SharedMemorySize(old_s, CallingHart()) < MessageDataLen(old_s) ==> ResultEqual(result, SBI_ERR_INVALID_PARAM))
    && (ResultEqual(result, SBI_SUCCESS) ==> MessageTransmitted(old_s, ChannelId(old_s), MessageId(old_s), SharedMemoryData(old_s, CallingHart(), 0, MessageDataLen(old_s))))
}