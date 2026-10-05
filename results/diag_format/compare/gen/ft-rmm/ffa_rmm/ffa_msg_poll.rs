pub open spec fn ffa_msg_poll_spec(caller: Caller, result: Result, old_s: S, new_s: S) -> bool {
  (!MessageAvailable(old_s, RxBuffer(new_s, caller)) ==> ResultEqual(result, RETRY))
  && (!CalleeCanHandleRequest(old_s) ==> ResultEqual(result, DENIED))
  && (!IsImplemented(old_s, FFA_MSG_POLL, ffa_instance) ==> ResultEqual(result, NOT_SUPPORTED))
  && (result == FFA_MSG_SEND ==> ResultEqual(result, FFA_MSG_SEND))
  && (result == FFA_MSG_SEND ==> MessageAvailable(new_s, RxBuffer(new_s, caller)))
  && ((MessageAvailable(old_s, RxBuffer(new_s, caller)) &&
       CalleeCanHandleRequest(old_s) &&
       IsImplemented(old_s, FFA_MSG_POLL, ffa_instance))
    ==> result == FFA_SUCCESS)
  && (result != FFA_MSG_SEND
    ==> !MessageAvailable(new_s, RxBuffer(new_s, caller)))
}