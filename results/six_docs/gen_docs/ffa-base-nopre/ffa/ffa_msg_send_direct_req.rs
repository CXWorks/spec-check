pub open spec fn ffa_msg_send_direct_req_spec(
    result: RsiCommandReturnCode,
    old_s: S,
    new_s: S,
) -> bool {
    // Failure conditions: Invalid endpoint ID or message flags
    (result == RSI_ERROR_INPUT ==> (
        !old_s.is_valid_endpoint_id(old_s.cmd_input_sender_id)
        || !old_s.is_valid_endpoint_id(old_s.cmd_input_receiver_id)
        || !old_s.is_valid_message_flags(old_s.cmd_input_flags)
    ))
    // Failure conditions: Callee not in a state to handle this request
    (result == RSI_ERROR_STATE ==> !old_s.is_callee_ready_to_handle_direct_request())
    // Failure conditions: Receiver endpoint does not support receipt of Direct request messages
    (result == RSI_ERROR_STATE ==> !old_s.receiver_endpoint_supports_direct_request())
    // Failure conditions: Function not implemented at this FF-A instance
    (result == RSI_ERROR_UNKNOWN ==> !old_s.is_ffa_msg_send_direct_req_implemented())
    // Failure conditions: Receiver endpoint is in a running, blocked or preempted state
    (result == RSI_ERROR_STATE ==> old_s.receiver_endpoint_state_is_busy())
    // Failure conditions: Receiver endpoint ran into an unexpected error and has aborted
    (result == RSI_ERROR_STATE ==> old_s.receiver_endpoint_state_is_aborted())
    // Failure conditions: Receiver endpoint is not ready to handle this request
    (result == RSI_ERROR_STATE ==> !old_s.receiver_endpoint_is_ready())
    // Success condition: Command returns RSI_SUCCESS
    (result == RSI_SUCCESS ==> (
        old_s.is_valid_endpoint_id(old_s.cmd_input_sender_id)
        && old_s.is_valid_endpoint_id(old_s.cmd_input_receiver_id)
        && old_s.is_valid_message_flags(old_s.cmd_input_flags)
        && old_s.is_callee_ready_to_handle_direct_request()
        && old_s.receiver_endpoint_supports_direct_request()
        && old_s.is_ffa_msg_send_direct_req_implemented()
        && !old_s.receiver_endpoint_state_is_busy()
        && !old_s.receiver_endpoint_state_is_aborted()
        && old_s.receiver_endpoint_is_ready()
    ))
}