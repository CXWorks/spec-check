pub open spec fn ffa_rxtx_unmap_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == FFA_INVALID_PARAMETERS ==> !old_s.has_rx_tx_buffer_pair())
    && (result == FFA_NOT_SUPPORTED ==> true)
    && (result == FFA_SUCCESS ==> old_s.has_rx_tx_buffer_pair() && !new_s.has_rx_tx_buffer_pair())
}