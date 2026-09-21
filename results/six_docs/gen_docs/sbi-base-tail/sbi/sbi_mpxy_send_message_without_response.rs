pub open spec fn sbi_mpxy_send_message_without_response_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_INVALID_PARAM ==> (old_s.cmd_input_message_data_len > 0))
    && (result == SBI_SBI_ERR_INVALID_PARAM ==> (old_s.cmd_input_message_data_len > old_s.shared_memory_size))
    && (result == SBI_SBI_ERR_INVALID_PARAM ==> (old_s.cmd_input_message_data_len > 0))
    && (result == SBI_SBI_SUCCESS ==> true)
}