pub open spec fn ffa_rxtx_unmap_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == FFA_INVALID_PARAMETERS ==> !old_s.rxtx_buffer_registered(old_s.id))
    && (result == FFA_NOT_SUPPORTED ==> !old_s.ffa_rxtx_unmap_supported())
    && (result == FFA_SUCCESS ==> old_s.rxtx_buffer_registered(old_s.id) && !new_s.rxtx_buffer_registered(old_s.id))
}