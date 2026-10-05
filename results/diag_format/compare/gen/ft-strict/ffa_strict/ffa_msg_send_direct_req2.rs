pub open spec fn ffa_msg_send_direct_req2_spec(sender_id: UInt16, receiver_id: UInt16, uuid_lo: UInt64, uuid_hi: UInt64, result: Int32, ret_fid: UInt32, old_s: S, new_s: S) -> bool {
  (!IsValidEndpointId(old_s, sender_id) || !IsValidEndpointId(old_s, receiver_id) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsRecognizedUuid(old_s, receiver_id, uuid_lo, uuid_hi) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!CalleeCanHandleRequest(old_s, receiver_id) ==> ResultEqual(result, DENIED))
  && (!CallerMayInvokeDirectReq2(old_s, sender_id) ==> ResultEqual(result, DENIED))
  && (!SupportsDirectReqReceipt(old_s, receiver_id) ==> ResultEqual(result, DENIED))
  && (!IsImplementedAtInstance(old_s, FFA_MSG_SEND_DIRECT_REQ2) ==> ResultEqual(result, NOT_SUPPORTED))
  && (RuntimeState(old_s, receiver_id) == RUNNING || RuntimeState(old_s, receiver_id) == BLOCKED || RuntimeState(old_s, receiver_id) == PREEMPTED ==> ResultEqual(result, BUSY))
  && (EndpointHasAborted(old_s, receiver_id) ==> ResultEqual(result, ABORTED))
  && (!EndpointReadyForRequest(old_s, receiver_id) ==> ResultEqual(result, NOT_READY))
  && (result == FFA_MSG_SEND_DIRECT_RESP2 || result == FFA_INTERRUPT || result == FFA_YIELD || result == FFA_SUCCESS)
  && (result == FFA_INTERRUPT ==> MustResumeViaFfaRun(new_s, receiver_id))
  && (result == FFA_YIELD ==> RuntimeState(new_s, receiver_id) == BLOCKED && MustResumeViaFfaRun(new_s, receiver_id))
  && (result == FFA_SUCCESS ==> OtherParamRegistersAreZero(new_s))
  && ((IsValidEndpointId(old_s, sender_id) &&
       IsRecognizedUuid(old_s, receiver_id, uuid_lo, uuid_hi) &&
       CalleeCanHandleRequest(old_s, receiver_id) &&
       CallerMayInvokeDirectReq2(old_s, sender_id) &&
       SupportsDirectReqReceipt(old_s, receiver_id) &&
       IsImplementedAtInstance(old_s, FFA_MSG_SEND_DIRECT_REQ2) &&
       !(RuntimeState(old_s, receiver_id) == RUNNING || RuntimeState(old_s, receiver_id) == BLOCKED || RuntimeState(old_s, receiver_id) == PREEMPTED) &&
       !(EndpointHasAborted(old_s, receiver_id)) &&
       EndpointReadyForRequest(old_s, receiver_id))
    ==> result == FFA_SUCCESS)
}