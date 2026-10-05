pub open spec fn ffa_msg_send_direct_req_spec(sender_id: UInt16, receiver_id: UInt16, msg_subtype: UInt8, reserved: UInt23, msg_type: UInt1, impdef: [UInt64; 5], impdef64: [UInt64; 10], result: Result<(), FfaStatusCode>, old_s: S, new_s: S) -> bool {
  (!IsValidEndpointId(old_s, sender_id) || !IsValidEndpointId(old_s, receiver_id) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!AreValidMessageFlags(old_s, msg_type, reserved, msg_subtype) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!CalleeCanHandleRequest(old_s) ==> ResultEqual(result, DENIED))
  && (!EndpointSupportsDirectReqReceipt(old_s, receiver_id) ==> ResultEqual(result, DENIED))
  && (!IsImplementedAtInstance(old_s, FFA_MSG_SEND_DIRECT_REQ) ==> ResultEqual(result, NOT_SUPPORTED))
  && (EndpointState(old_s, receiver_id) in {RUNNING, BLOCKED, PREEMPTED} ==> ResultEqual(result, BUSY))
  && (EndpointHasAborted(old_s, receiver_id) ==> ResultEqual(result, ABORTED))
  && (!EndpointIsReady(old_s, receiver_id) ==> ResultEqual(result, NOT_READY))
  && (result == FFA_MSG_SEND_DIRECT_RESP ==> result == FFA_MSG_SEND_DIRECT_RESP)
  && (result == FFA_INTERRUPT ==> result == FFA_INTERRUPT)
  && (result == FFA_YIELD ==> result == FFA_YIELD)
  && (result == FFA_SUCCESS ==> result == FFA_SUCCESS)
  && ((IsValidEndpointId(old_s, sender_id) &&
       AreValidMessageFlags(old_s, msg_type, reserved, msg_subtype) &&
       CalleeCanHandleRequest(old_s) &&
       EndpointSupportsDirectReqReceipt(old_s, receiver_id) &&
       IsImplementedAtInstance(old_s, FFA_MSG_SEND_DIRECT_REQ) &&
       !(EndpointState(old_s, receiver_id) in {RUNNING, BLOCKED, PREEMPTED}) &&
       !(EndpointHasAborted(old_s, receiver_id)) &&
       EndpointIsReady(old_s, receiver_id))
    ==> result == FFA_SUCCESS)
}