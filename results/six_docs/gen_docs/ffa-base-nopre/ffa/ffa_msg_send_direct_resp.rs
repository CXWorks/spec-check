pub open spec fn ffa_msg_send_direct_resp_spec(result: u32, old_s: S, new_s: S) -> bool {
    (result == FFA_ERROR_INVALID_PARAMETERS ==> (old_s.source_id as int < 0 || old_s.source_id as int > 0xFFFF || old_s.dest_id as int < 0 || old_s.dest_id as int > 0xFFFF || (old_s.flags as int & 0xFF00_0000) != 0))
    && (result == FFA_ERROR_DENIED ==> true)
    && (result == FFA_ERROR_NOT_SUPPORTED ==> true)
    && (result == FFA_ERROR_ABORTED ==> true)
    && (result == 0 ==> true)
}