pub open spec fn ffa_msg_send_direct_req2_spec(result: int, old_s: S, new_s: S) -> bool {
    // Failure conditions
    // Invalid endpoint ID or message flags
    (old_s.ffa_msg_send_direct_req2_sender_id as int < 0 || old_s.ffa_msg_send_direct_req2_sender_id as int > 0xFFFF ||
     old_s.ffa_msg_send_direct_req2_receiver_id as int < 0 || old_s.ffa_msg_send_direct_req2_receiver_id as int > 0xFFFF) ==> result == FFA_ERROR_INVALID_PARAMETERS
    &&
    // Unrecognized UUID
    (old_s.ffa_msg_send_direct_req2_uuid_lo != 0 || old_s.ffa_msg_send_direct_req2_uuid_hi != 0) ==> result == FFA_ERROR_INVALID_PARAMETERS
    &&
    // Callee is not in a state to handle this request
    (old_s.ffa_msg_send_direct_req2_state != FFA_STATE_READY) ==> result == FFA_ERROR_DENIED
    &&
    // Caller is not allowed to invoke this ABI
    (old_s.ffa_msg_send_direct_req2_instance not in [FFA_INSTANCE_NON_SECURE_PHYSICAL, FFA_INSTANCE_SECURE_PHYSICAL, FFA_INSTANCE_NON_SECURE_VIRTUAL, FFA_INSTANCE_SECURE_VIRTUAL]) ==> result == FFA_ERROR_DENIED
    &&
    // Receiver endpoint does not support receipt of Direct request messages
    (old_s.ffa_msg_send_direct_req2_receiver_supports_direct == false) ==> result == FFA_ERROR_DENIED
    &&
    // This function is not implemented at this FF-A instance
    (old_s.ffa_msg_send_direct_req2_supported == false) ==> result == FFA_ERROR_NOT_SUPPORTED
    &&
    // Receiver endpoint is in a running, blocked or preempted state
    (old_s.ffa_msg_send_direct_req2_state == FFA_STATE_RUNNING || old_s.ffa_msg_send_direct_req2_state == FFA_STATE_BLOCKED || old_s.ffa_msg_send_direct_req2_state == FFA_STATE_PREEMPTED) ==> result == FFA_ERROR_BUSY
    &&
    // Receiver endpoint ran into an unexpected error and has aborted
    (old_s.ffa_msg_send_direct_req2_state == FFA_STATE_ABORTED) ==> result == FFA_ERROR_ABORTED
    &&
    // Receiver endpoint is not ready to handle this request
    (old_s.ffa_msg_send_direct_req2_state == FFA_STATE_NOT_READY) ==> result == FFA_ERROR_NOT_READY
    // Success conditions
    // Successful completion is indicated through an invocation of the following interfaces by the callee:
    // – FFA_MSG_SEND_DIRECT_RESP2 to provide a response to the Direct request.
    // – FFA_INTERRUPT to indicate that the Direct request was interrupted and must be resumed through the FFA_RUN interface.
    // – FFA_YIELD to indicate that the Receiver endpoint has transitioned to the blocked runtime state and must be resumed through the FFA_RUN interface.
    // – FFA_SUCCESS to indicate completion of the Direct request without a corresponding Direct response.
    // All other parameter registers MBZ.
    (result == FFA_SUCCESS || result == FFA_ERROR_INVALID_PARAMETERS || result == FFA_ERROR_DENIED || result == FFA_ERROR_NOT_SUPPORTED || result == FFA_ERROR_BUSY || result == FFA_ERROR_ABORTED || result == FFA_ERROR_NOT_READY) ==> true
}