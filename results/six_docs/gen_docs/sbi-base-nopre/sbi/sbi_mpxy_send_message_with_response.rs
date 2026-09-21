pub open spec fn sbi_mpxy_send_message_with_response_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.error == SBI_SUCCESS ==> result.uvalue == (new_s.mpxy_response_data_len as int))
    && (result.error == SBI_SUCCESS ==> new_s.mpxy_response_data_len > 0)
    && (result.error != SBI_SUCCESS ==> result.uvalue == 0)
    && (result.error != SBI_SUCCESS ==> new_s.mpxy_response_data_len == 0)
}