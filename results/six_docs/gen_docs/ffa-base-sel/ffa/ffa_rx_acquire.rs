pub open spec fn ffa_rx_acquire_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == FFA_INVALID_PARAMETERS ==> !old_s.has_rx_buffer_pair(old_s.vm_id))
    && (result == FFA_DENIED ==> old_s.has_rx_buffer_pair(old_s.vm_id) && !old_s.can_relinquish_ownership(old_s.vm_id))
    && (result == FFA_NOT_SUPPORTED ==> !old_s.is_ffa_instance_supported(old_s.vm_id))
    && (result == FFA_SUCCESS ==> old_s.vm_id == new_s.vm_id && old_s.has_rx_buffer_pair(old_s.vm_id) && old_s.can_relinquish_ownership(old_s.vm_id))
}