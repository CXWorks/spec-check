pub open spec fn sbi_mpxy_send_message_without_response_spec(result: SbiRet, old_s: S, new_s: S, channel_id: UInt32, message_id: UInt32, message_data_len: UInt64) -> bool {
    (((message_data_len as int) > (MpxyMsgDataMaxLen(old_s, channel_id) as int)) ==> (result.error == SBI_ERR_INVALID_PARAM && new_s == old_s))
    && (((message_data_len as int) > (MpxyShmemSize(old_s) as int)) ==> (result.error == SBI_ERR_INVALID_PARAM && new_s == old_s))
    && ((((message_data_len as int) <= (MpxyMsgDataMaxLen(old_s, channel_id) as int)) && ((message_data_len as int) <= (MpxyShmemSize(old_s) as int))) ==> (result.error == SBI_SUCCESS && MpxyPostedMessageSent(old_s, new_s, channel_id, message_id, message_data_len)))
}
