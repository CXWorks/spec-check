pub open spec fn ffa_msg_send_direct_req2_spec(sender_id: UInt16, receiver_id: UInt16, uuid_lo: UInt64, uuid_hi: UInt64, result: Result<(), Int32>, old_s: S, new_s: S) -> bool {
  (!IsValidEndpointId(old_s, sender_id) || !IsValidEndpointId(old_s, receiver_id) || !AreValidMessageFlags(old_s) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsRecognizedUuid(old_s, uuid_lo, uuid_hi) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!CalleeCanHandleRequest(old_s) ==> ResultEqual(result, DENIED))
  && (!CallerMayInvoke(old_s, FFA_MSG_SEND_DIRECT_REQ2) ==> ResultEqual(result, DENIED))
  && (!EndpointSupportsDirectReq(old_s, receiver_id) ==> ResultEqual(result, DENIED))
  && (!IsImplementedAtInstance(old_s, FFA_MSG_SEND_DIRECT_REQ2) ==> ResultEqual(result, NOT_SUPPORTED))
  && (EndpointAt(old_s, receiver_id).state in {RUNNING, BLOCKED, PREEMPTED} ==> ResultEqual(result, BUSY))
  && (EndpointAt(old_s, receiver_id).state == ABORTED ==> ResultEqual(result, ABORTED))
  && (!EndpointIsReady(old_s, receiver_id) ==> ResultEqual(result, NOT_READY))
  && (result == FFA_MSG_SEND_DIRECT_RESP2 ==> CalleeInvoked(new_s) == FFA_MSG_SEND_DIRECT_RESP2)
  && (result == FFA_INTERRUPT ==> CalleeInvoked(new_s) == FFA_INTERRUPT)
  && (result == FFA_YIELD ==> CalleeInvoked(new_s) == FFA_YIELD && EndpointAt(new_s, receiver_id).state == BLOCKED)
  && (result == FFA_SUCCESS ==> CalleeInvoked(new_s) == FFA_SUCCESS)
  && ((IsValidEndpointId(old_s, sender_id) &&
       IsRecognizedUuid(old_s, uuid_lo, uuid_hi) &&
       CalleeCanHandleRequest(old_s) &&
       CallerMayInvoke(old_s, FFA_MSG_SEND_DIRECT_REQ2) &&
       EndpointSupportsDirectReq(old_s, receiver_id) &&
       IsImplementedAtInstance(old_s, FFA_MSG_SEND_DIRECT_REQ2) &&
       !(EndpointAt(old_s, receiver_id).state in {RUNNING, BLOCKED, PREEMPTED}) &&
       !(EndpointAt(old_s, receiver_id).state == ABORTED) &&
       EndpointIsReady(old_s, receiver_id))
    ==> result == FFA_SUCCESS)
  && (result != FFA_MSG_SEND_DIRECT_RESP2 &&
       result != FFA_INTERRUPT &&
       result != FFA_YIELD &&
       result != FFA_SUCCESS
    ==> CalleeInvoked(new_s) == FFA_SUCCESS)
  && (CalleeInvoked(new_s) != FFA_MSG_SEND_DIRECT_RESP2 &&
       CalleeInvoked(new_s) != FFA_INTERRUPT &&
       CalleeInvoked(new_s) != FFA_YIELD &&
       CalleeInvoked(new_s) != FFA_SUCCESS
    ==> EndpointAt(new_s, receiver_id).state == EndpointAt(old_s, receiver_id).state)
  && (result == FFA_SUCCESS
    ==> EndpointAt(new_s, receiver_id).state == BLOCKED)
  && ((!(result == FFA_MSG_SEND_DIRECT_RESP2) &&
       !(result == FFA_INTERRUPT) &&
       !(result == FFA_YIELD) &&
       !(result == FFA_SUCCESS))
    ==> EndpointAt(new_s, receiver_id).state == EndpointAt(old_s, receiver_id).state)
}