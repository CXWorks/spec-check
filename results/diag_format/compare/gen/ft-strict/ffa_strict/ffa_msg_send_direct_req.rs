pub open spec fn ffa_msg_send_direct_req_spec(sender_receiver_ids: UInt32, flags: UInt32, impdef_args: [UInt32; 5], impdef_args64: [UInt64; 14], result: UInt32, error_code: Int32, old_s: S, new_s: S) -> bool {
  (!IsImplementedAtInstance(old_s, FFA_MSG_SEND_DIRECT_REQ, CurrentFfaInstance(old_s)) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsValidEndpointId(old_s, Bits(sender_receiver_ids, 31, 16)) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsValidEndpointId(old_s, Bits(sender_receiver_ids, 15, 0)) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (Bits(flags, 30, 8) != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (Bits(flags, 31, 31) == 0 && Bits(flags, 7, 0) != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (Bits(flags, 31, 31) == 1 && !IsValidFrameworkMessageType(old_s, Bits(flags, 7, 0)) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!CalleeCanHandleRequest(old_s) ==> ResultEqual(result, DENIED))
  && (!SupportsDirectRequestReceipt(old_s, Bits(sender_receiver_ids, 15, 0)) ==> ResultEqual(result, DENIED))
  && (Endpoint(old_s, Bits(sender_receiver_ids, 15, 0)).runtime_state == RUNNING || Endpoint(old_s, Bits(sender_receiver_ids, 15, 0)).runtime_state == BLOCKED || Endpoint(old_s, Bits(sender_receiver_ids, 15, 0)).runtime_state == PREEMPTED ==> ResultEqual(result, BUSY))
  && (EndpointHasAborted(old_s, Bits(sender_receiver_ids, 15, 0)) ==> ResultEqual(result, ABORTED))
  && (!EndpointIsReady(old_s, Bits(sender_receiver_ids, 15, 0)) ==> ResultEqual(result, NOT_READY))
  && (ResultEqual(result, FFA_MSG_SEND_DIRECT_RESP) || ResultEqual(result, FFA_INTERRUPT) || ResultEqual(result, FFA_YIELD) || ResultEqual(result, FFA_SUCCESS) ==> true)
  && (ResultEqual(result, FFA_MSG_SEND_DIRECT_RESP) ==> IsDirectResponseTo(new_s, Bits(sender_receiver_ids, 31, 16), Bits(sender_receiver_ids, 15, 0)))
  && (ResultEqual(result, FFA_INTERRUPT) ==> MustResumeViaFfaRun(new_s, Bits(sender_receiver_ids, 15, 0)))
  && (ResultEqual(result, FFA_YIELD) ==> Endpoint(new_s, Bits(sender_receiver_ids, 15, 0)).runtime_state == BLOCKED)
  && (ResultEqual(result, FFA_YIELD) ==> MustResumeViaFfaRun(new_s, Bits(sender_receiver_ids, 15, 0)))
  && (ResultEqual(result, FFA_SUCCESS) ==> OtherParameterRegistersAreZero(new_s))
  && ((IsImplementedAtInstance(old_s, FFA_MSG_SEND_DIRECT_REQ, CurrentFfaInstance(old_s)) &&
       IsValidEndpointId(old_s, Bits(sender_receiver_ids, 31, 16)) &&
       IsValidEndpointId(old_s, Bits(sender_receiver_ids, 15, 0)) &&
       !(Bits(flags, 30, 8) != 0) &&
       !(Bits(flags, 31, 31) == 0 && Bits(flags, 7, 0) != 0) &&
       !(Bits(flags, 31, 31) == 1 && !IsValidFrameworkMessageType(old_s, Bits(flags, 7, 0))) &&
       CalleeCanHandleRequest(old_s) &&
       SupportsDirectRequestReceipt(old_s, Bits(sender_receiver_ids, 15, 0)) &&
       !(Endpoint(old_s, Bits(sender_receiver_ids, 15, 0)).runtime_state == RUNNING || Endpoint(old_s, Bits(sender_receiver_ids, 15, 0)).runtime_state == BLOCKED || Endpoint(old_s, Bits(sender_receiver_ids, 15, 0)).runtime_state == PREEMPTED) &&
       !(EndpointHasAborted(old_s, Bits(sender_receiver_ids, 15, 0))) &&
       EndpointIsReady(old_s, Bits(sender_receiver_ids, 15, 0)))
    ==> ResultEqual(result, FFA_SUCCESS))
}