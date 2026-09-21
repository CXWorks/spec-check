pub open spec fn ffa_rx_acquire_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == FFA_DENIED ==> true)
    && (result == FFA_INVALID_PARAMETERS ==> true)
    && (result == FFA_NOT_SUPPORTED ==> true)
    && (result == FFA_SUCCESS ==> true)
}