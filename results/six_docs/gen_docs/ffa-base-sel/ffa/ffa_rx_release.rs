pub open spec fn ffa_rx_release_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == FFA_DENIED ==> (old_s.ffa_vm_id != new_s.ffa_vm_id))
    && (result == FFA_INVALID_PARAMETERS ==> (old_s.ffa_vm_id == new_s.ffa_vm_id))
    && (result == FFA_NOT_SUPPORTED ==> (old_s.ffa_vm_id == new_s.ffa_vm_id))
    && (result == FFA_SUCCESS ==> (old_s.ffa_vm_id == new_s.ffa_vm_id))
}