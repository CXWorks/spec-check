pub open spec fn sbi_mpxy_send_message_without_response_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.error == SBI_ERR_INVALID_PARAM ==> (old_s.mpxy_channels[old_s.channel_id].message_data_len > MSG_DATA_MAX_LEN || old_s.mpxy_channels[old_s.channel_id].message_data_len > old_s.hart_shared_memory_size))
    && (result.error == SBI_SUCCESS ==> (old_s.mpxy_channels[old_s.channel_id].message_data_len <= MSG_DATA_MAX_LEN && old_s.mpxy_channels[old_s.channel_id].message_data_len <= old_s.hart_shared_memory_size))
    && (result.error == SBI_SUCCESS ==> new_s.mpxy_channels[old_s.channel_id].message_data_len == old_s.mpxy_channels[old_s.channel_id].message_data_len)
    && (result.error == SBI_SUCCESS ==> new_s.hart_shared_memory_size == old_s.hart_shared_memory_size)
    && (result.error == SBI_SUCCESS ==> new_s.mpxy_channels[old_s.channel_id].message_data_len == old_s.mpxy_channels[old_s.channel_id].message_data_len)
}