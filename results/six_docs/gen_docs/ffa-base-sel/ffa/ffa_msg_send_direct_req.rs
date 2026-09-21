pub open spec fn ffa_msg_send_direct_req_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == FFA_INVALID_PARAMETERS ==> (old_s.sender_id as int < 0 || old_s.sender_id as int > 0xFFFF || old_s.receiver_id as int < 0 || old_s.receiver_id as int > 0xFFFF))
    && (result == FFA_DENIED ==> true)
    && (result == FFA_NOT_SUPPORTED ==> true)
    && (result == FFA_BUSY ==> true)
    && (result == FFA_ABORTED ==> true)
    && (result == FFA_NOT_READY ==> true)
    && (result == FFA_SUCCESS ==> true)
    && (result == FFA_RETRY ==> true)
    && (result == FFA_NO_DATA ==> true)
    && (result == FFA_NO_MEMORY ==> true)
    && (result == FFA_INTERRUPTED ==> true)
    && (result == FFA_YIELD ==> true)
}