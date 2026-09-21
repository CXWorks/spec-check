pub open spec fn ffa_rx_acquire_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == FFA_SUCCESS ==> true)
    && (result == FFA_ERROR_INVALID_PARAMETERS ==> !IsVmRegistered(old_s, vm_id))
    && (result == FFA_ERROR_NOT_SUPPORTED ==> !IsFfaInstanceSupported(old_s, ff_a_instance))
    && (result == FFA_ERROR_DENIED ==> true)
}