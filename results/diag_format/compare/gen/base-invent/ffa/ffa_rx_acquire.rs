pub open spec fn ffa_rx_acquire_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == FFA_SUCCESS ==> true)
    && (result == FFA_ERROR(FFA_DENIED) ==> !old_s.vm_rx_buffer_owned(old_s.vm_id))
    && (result == FFA_ERROR(FFA_INVALID_PARAMETERS) ==> !old_s.vm_rx_buffer_registered(old_s.vm_id))
    && (result == FFA_ERROR(FFA_NOT_SUPPORTED) ==> true)
}