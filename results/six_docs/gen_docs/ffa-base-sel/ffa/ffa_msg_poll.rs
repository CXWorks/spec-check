pub open spec fn ffa_msg_poll_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == FFA_RETRY)
    || (result == FFA_DENIED)
    || (result == FFA_NOT_SUPPORTED)
}