pub open spec fn ffa_msg_poll_spec(caller: Caller, callee: Callee, result: Result, old_s: S, new_s: S) -> bool {
  (!MessageAvailableInRxBuffer(old_s, caller) ==> ResultEqual(result, RETRY))
  && (!CalleeInStateToHandleRequest(old_s, callee) ==> ResultEqual(result, DENIED))
  && (!IsImplementedAtInstance(old_s, FFA_MSG_POLL, ffa_instance) ==> ResultEqual(result, NOT_SUPPORTED))
  && (result == FFA_MSG_SEND ==> ResultEqual(result, FFA_MSG_SEND))
  && ((MessageAvailableInRxBuffer(old_s, caller) &&
       CalleeInStateToHandleRequest(old_s, callee) &&
       IsImplementedAtInstance(old_s, FFA_MSG_POLL, ffa_instance))
    ==> result == FFA_MSG_SEND)
  && (result != FFA_MSG_SEND
    ==> result == RETRY)
  && (result != FFA_MSG_SEND
    ==> result == DENIED)
  && (result != FFA_MSG_SEND
    ==> result == NOT_SUPPORTED)
  && (result == FFA_ERROR
    ==> result == RETRY)
  && (result == FFA_ERROR
    ==> result == DENIED)
  && (result == FFA_ERROR
    ==> result == NOT_SUPPORTED)
  && (result != FFA_MSG_SEND && result != FFA_ERROR
    ==> true)
}