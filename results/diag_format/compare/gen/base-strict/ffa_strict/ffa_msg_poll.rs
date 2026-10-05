pub open spec fn ffa_msg_poll_spec(result: UInt32, old_s: S, new_s: S) -> bool {
    (!MessageAvailableInRxBuffer(old_s, caller) ==> ResultEqual(result, RETRY))
    && (!CalleeInStateToHandleRequest(old_s, callee) ==> ResultEqual(result, DENIED))
    && (!IsImplementedAtInstance(old_s, FFA_MSG_POLL, ffa_instance) ==> ResultEqual(result, NOT_SUPPORTED))
    && (ResultEqual(result, FFA_MSG_SEND) ==> true)
    && (old_s == new_s)
}