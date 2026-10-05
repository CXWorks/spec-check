pub open spec fn ffa_msg_send_direct_resp2_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == FFA_ERROR_INVALID_PARAMETERS ==> (old_s.source_endpoint_id < 0 || old_s.source_endpoint_id > 0xFFFF || old_s.destination_endpoint_id < 0 || old_s.destination_endpoint_id > 0xFFFF))
    && (result == FFA_ERROR_DENIED ==> (true))
    && (result == FFA_ERROR_NOT_SUPPORTED ==> (true))
    && (result == FFA_ERROR_ABORTED ==> (true))
    && (result == FFA_SUCCESS ==> (true))
}