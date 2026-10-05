pub open spec fn ffa_msg_send_direct_resp_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == INVALID_PARAMETERS ==> (old_s.source_endpoint_id == 0 || old_s.source_endpoint_id > 0xFFFF || old_s.dest_endpoint_id == 0 || old_s.dest_endpoint_id > 0xFFFF || (old_s.flags & 0xFF) != 0))
    && (result == DENIED ==> true)
    && (result == NOT_SUPPORTED ==> true)
    && (result == ABORTED ==> true)
    && (result != INVALID_PARAMETERS && result != DENIED && result != NOT_SUPPORTED && result != ABORTED ==> true)
}