pub open spec fn ffa_msg_send_direct_resp2_spec(src_id: UInt16, dst_id: UInt16, result: Result<int, FfaStatusCode>, old_s: S, new_s: S) -> bool {
  (!IsValidEndpointId(old_s, src_id) || !IsValidEndpointId(old_s, dst_id) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!AreValidMessageFlags(old_s) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!CalleeCanHandleRequest(old_s) ==> ResultEqual(result, DENIED))
  && (!CallerSupportsDirectRespSend(old_s, src_id) ==> ResultEqual(result, DENIED))
  && (!ReceiverSupportsDirectRespReceipt(old_s, dst_id) ==> ResultEqual(result, DENIED))
  && (!IsImplementedAtInstance(old_s, FFA_MSG_SEND_DIRECT_RESP2) ==> ResultEqual(result, NOT_SUPPORTED))
  && (ReceiverAbortedOnUnexpectedError(old_s, dst_id) ==> ResultEqual(result, ABORTED))
  && (result == FFA_SUCCESS ==> DirectRespMessageDeliveredTo(new_s, dst_id))
  && (result == FFA_SUCCESS ==> EndpointRun(new_s, dst_id))
  && (result == FFA_SUCCESS ==> EndpointWaitsForNewMessage(new_s, src_id))
  && (result == FFA_SUCCESS ==> SuccessIndicatedAsFfaMsgWait(new_s, result))
  && ((IsValidEndpointId(old_s, src_id) &&
       AreValidMessageFlags(old_s) &&
       CalleeCanHandleRequest(old_s) &&
       CallerSupportsDirectRespSend(old_s, src_id) &&
       ReceiverSupportsDirectRespReceipt(old_s, dst_id) &&
       IsImplementedAtInstance(old_s, FFA_MSG_SEND_DIRECT_RESP2) &&
       !ReceiverAbortedOnUnexpectedError(old_s, dst_id))
    ==> result == FFA_SUCCESS)
  && (result != FFA_SUCCESS
    ==> DirectRespMessageDeliveredTo(new_s, dst_id))
  && (result != FFA_SUCCESS
    ==> EndpointRun(new_s, dst_id))
  && (result != FFA_SUCCESS
    ==> EndpointWaitsForNewMessage(new_s, src_id))
  && (result != FFA_SUCCESS
    ==> SuccessIndicatedAsFfaMsgWait(new_s, result))
}