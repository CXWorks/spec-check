pub open spec fn ffa_msg_send_direct_resp2_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_ERROR_INPUT ==> (old_s.source_id as int < 0 || old_s.source_id as int >= 0x1_0000_0000 || old_s.dest_id as int < 0 || old_s.dest_id as int >= 0x1_0000_0000))
    && (result == RSI_ERROR_STATE ==> !old_s.is_in_state_to_handle_direct_response(old_s.source_id, old_s.dest_id))
    && (result == RSI_ERROR_UNKNOWN ==> !old_s.supports_sending_direct_response(old_s.source_id, old_s.dest_id))
    && (result == RSI_ERROR_UNKNOWN ==> !old_s.receiver_supports_direct_response(old_s.source_id, old_s.dest_id))
    && (result == RSI_ERROR_UNKNOWN ==> !old_s.implements_ffa_msg_send_direct_resp2(old_s.source_id, old_s.dest_id))
    && (result == RSI_SUCCESS ==> old_s.source_id == new_s.source_id && old_s.dest_id == new_s.dest_id)
}