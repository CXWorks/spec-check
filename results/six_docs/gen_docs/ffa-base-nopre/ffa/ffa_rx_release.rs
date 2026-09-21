pub open spec fn ffa_rx_release_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == FFA_SUCCESS ==> (old_s.ffa_rx_release_vm_id == new_s.ffa_rx_release_vm_id))
    && (result == FFA_ERROR_DENIED ==> (old_s.ffa_rx_release_vm_id != new_s.ffa_rx_release_vm_id))
    && (result == FFA_ERROR_INVALID_PARAMETERS ==> (old_s.ffa_rx_release_vm_id == new_s.ffa_rx_release_vm_id))
    && (result == FFA_ERROR_NOT_SUPPORTED ==> (old_s.ffa_rx_release_vm_id == new_s.ffa_rx_release_vm_id))
}