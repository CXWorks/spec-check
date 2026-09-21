pub open spec fn ffa_msg_send2_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == FFA_INVALID_PARAMETERS ==> (
        (old_s.is_virtual() ==> old_s.sender_vm_id != 0)
        || (old_s.is_secure_physical() ==> old_s.sender_vm_id != 0)
        || (old_s.is_non_secure_physical() ==> old_s.sender_vm_id == 0)
        || (old_s.is_non_secure_physical() && old_s.partition_message_header().sender_id != 0)
        || (old_s.is_non_secure_physical() && old_s.partition_message_header().offset < old_s.partition_message_header().header_size)
        || (old_s.is_non_secure_physical() && old_s.partition_message_header().payload_size > old_s.tx_buffer_size())
        || (old_s.is_non_secure_physical() && old_s.partition_message_header().uuid != 0)
    ))
    && (result == FFA_BUSY ==> old_s.receiver_rx_buffer_free == false)
    && (result == FFA_DENIED ==> (
        (old_s.is_virtual() && old_s.callee_state != old_s.callee_state_ready)
        || (old_s.is_non_secure_physical() && old_s.caller_state != old_s.caller_state_allowed)
        || (old_s.is_virtual() && !old_s.receiver_endpoint_supports_indirect_messaging())
    ))
    && (result == FFA_NO_MEMORY ==> old_s.receiver_rx_buffer_free_space < old_s.partition_message_header().payload_size)
    && (result == FFA_NOT_SUPPORTED ==> !old_s.function_implemented())
    && (result == FFA_SUCCESS ==> true)
}