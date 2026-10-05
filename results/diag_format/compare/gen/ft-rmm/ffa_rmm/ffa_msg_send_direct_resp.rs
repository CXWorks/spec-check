pub open spec fn ffa_msg_send_direct_resp_spec(source_id: UInt16, dest_id: UInt16, msg_type: UInt, flags_rsvd: UInt, fwk_msg_type: UInt, result: Result<(), FfaCommandReturnCode>, error_code: Int32, old_s: S, new_s: S) -> bool {
  (!IsValidEndpointId(old_s, source_id) || !IsValidEndpointId(old_s, dest_id) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!AreValidMessageFlags(old_s, msg_type, flags_rsvd, fwk_msg_type) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!CalleeCanHandleRequest(old_s) ==> ResultEqual(result, DENIED))
  && (!CallerMayInvokeAbi(old_s, FFA_MSG_SEND_DIRECT_RESP) ==> ResultEqual(result, DENIED))
  && (!EndpointSupportsDirectRespReceipt(old_s, dest_id) ==> ResultEqual(result, DENIED))
  && (!IsImplementedAtInstance(old_s, FFA_MSG_SEND_DIRECT_RESP) ==> ResultEqual(result, NOT_SUPPORTED))
  && (EndpointAborted(old_s, dest_id) ==> ResultEqual(result, ABORTED))
  && (result == FFA_SUCCESS ==> DirectRespDelivered(new_s, source_id, dest_id, msg_type, fwk_msg_type, impdef_args))
  && (result == FFA_SUCCESS ==> EndpointRan(new_s, dest_id))
  && (result == FFA_SUCCESS ==> NewMessageAvailableFor(new_s, source_id))
  && (result == FFA_SUCCESS ==> SuccessReportedAsMsgWait(new_s, result))
  && ((IsValidEndpointId(old_s, source_id) &&
       AreValidMessageFlags(old_s, msg_type, flags_rsvd, fwk_msg_type) &&
       CalleeCanHandleRequest(old_s) &&
       CallerMayInvokeAbi(old_s, FFA_MSG_SEND_DIRECT_RESP) &&
       EndpointSupportsDirectRespReceipt(old_s, dest_id) &&
       IsImplementedAtInstance(old_s, FFA_MSG_SEND_DIRECT_RESP) &&
       !(EndpointAborted(old_s, dest_id)))
    ==> result == FFA_SUCCESS)
  && (result != FFA_SUCCESS
    ==> Endpoint(dest_id).message == Endpoint(old_s, dest_id).message)
  && (result != FFA_SUCCESS
    ==> Endpoint(dest_id).run_state == Endpoint(old_s, dest_id).run_state)
  && (result != FFA_SUCCESS
    ==> Endpoint(source_id).run_state == Endpoint(old_s, source_id).run_state)
}