pub open spec fn ffa_msg_send_direct_req2_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_ERROR_INPUT ==> (
        (old_s.sender_id as int) < 0 || (old_s.sender_id as int) > 0xFFFF ||
        (old_s.receiver_id as int) < 0 || (old_s.receiver_id as int) > 0xFFFF ||
        (old_s.uuid_lo as int) == 0 || (old_s.uuid_hi as int) == 0
    ))
    && (result == RSI_ERROR_STATE ==> (
        // Callee is not in a state to handle this request
        true
    ))
    && (result == RSI_ERROR_UNKNOWN ==> (
        // Caller is not allowed to invoke this ABI
        true
    ))
    && (result == RSI_ERROR_UNKNOWN ==> (
        // Receiver endpoint does not support receipt of Direct request messages
        true
    ))
    && (result == RSI_ERROR_UNKNOWN ==> (
        // This function is not implemented at this FF-A instance
        true
    ))
    && (result == RSI_ERROR_UNKNOWN ==> (
        // Receiver endpoint is in a running, blocked or preempted state
        true
    ))
    && (result == RSI_ERROR_UNKNOWN ==> (
        // Receiver endpoint ran into an unexpected error and has aborted
        true
    ))
    && (result == RSI_ERROR_UNKNOWN ==> (
        // Receiver endpoint is not ready to handle this request
        true
    ))
    && (result == RSI_SUCCESS ==> (
        // Successful completion: response provided via FFA_MSG_SEND_DIRECT_RESP2,
        // interrupted via FFA_INTERRUPT, yielded via FFA_YIELD, or completed without response
        true
    ))
}