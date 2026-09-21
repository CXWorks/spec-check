pub open spec fn sbi_mpxy_send_message_without_response_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_INVALID_PARAM ==> (old_s.mpxy_channel_data_len(old_s.mpxy_channel_id) > MSG_DATA_MAX_LEN || old_s.mpxy_channel_data_len(old_s.mpxy_channel_id) > old_s.shared_memory_size(old_s.mpxy_channel_id)))
    && (result == SBI_SBI_SUCCESS ==> true)
}