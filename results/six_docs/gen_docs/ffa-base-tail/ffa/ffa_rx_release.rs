pub open spec fn ffa_rx_release_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == FFA_DENIED ==> true)
    && (result == FFA_INVALID_PARAMETERS ==> true)
    && (result == FFA_NOT_SUPPORTED ==> true)
    && (result == FFA_SUCCESS ==> true)
}