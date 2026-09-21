pub open spec fn ffa_msg_poll_spec(result: int32, old_s: S, new_s: S) -> bool {
    (!IsDirectRequest(old_s) ==> (result == RETRY || result == DENIED || result == NOT_SUPPORTED))
    && (IsDirectRequest(old_s) ==> result == DENIED)
}