pub open spec fn ffa_msg_poll_spec(result: int32, old_s: S, new_s: S) -> bool {
    (!old_s.is_processing_direct_request() ==> result == FFA_RETRY)
    && (old_s.is_processing_direct_request() ==> result == FFA_DENIED)
    && (result == FFA_RETRY || result == FFA_DENIED)
}