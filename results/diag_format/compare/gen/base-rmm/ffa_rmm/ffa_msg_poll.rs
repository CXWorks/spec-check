pub open spec fn ffa_msg_poll_spec(result: UInt32, old_s: S, new_s: S) -> bool {
    (!MessageAvailable(old_s, RxBuffer(caller)) ==> ResultEqual(result, RETRY))
    && (!CalleeCanHandleRequest(old_s) ==> ResultEqual(result, DENIED))
    && (!IsImplemented(FFA_MSG_POLL, ffa_instance(old_s)) ==> ResultEqual(result, NOT_SUPPORTED))
    && (ResultEqual(result, FFA_MSG_SEND) ==> MessageAvailable(new_s, RxBuffer(caller)))
}