pub open spec fn ffa_msg_send_direct_resp_spec(ids: UInt32, flags: UInt32, result: Int32, old_s: S, new_s: S) -> bool {
  (!IsValidEndpointId(old_s, Bits(ids, 31, 16)) || !IsValidEndpointId(old_s, Bits(ids, 15, 0))) ==> ResultEqual(result, INVALID_PARAMETERS)
  && (!IsValidMessageFlags(old_s, flags)) ==> ResultEqual(result, INVALID_PARAMETERS)
  && (!CalleeCanHandleRequest(old_s, Bits(ids, 15, 0))) ==> ResultEqual(result, DENIED)
  && (!CallerAllowedToInvoke(old_s, Bits(ids, 31, 16), 0x84000070)) ==> ResultEqual(result, DENIED)
  && (!SupportsDirectResponseReceipt(old_s, Bits(ids, 15, 0))) ==> ResultEqual(result, DENIED)
  && (!IsImplementedAtFfaInstance(old_s, 0x84000070)) ==> ResultEqual(result, NOT_SUPPORTED)
  && (ReceiverEncounteredUnexpectedError(old_s, Bits(ids, 15, 0))) ==> ResultEqual(result, ABORTED)
  && (result == FFA_SUCCESS) ==> CompletesAsFfaMsgWait(new_s, result)
  && (result == FFA_SUCCESS) ==> DirectResponseDelivered(new_s, Bits(ids, 15, 0), Bits(ids, 31, 16), flags)
  && (result == FFA_SUCCESS) ==> EndpointRuns(new_s, Bits(ids, 15, 0))
  && (result == FFA_SUCCESS) ==> EndpointWaitsForNewMessage(new_s, Bits(ids, 31, 16))
  && ((IsValidEndpointId(old_s, Bits(ids, 31, 16)) && IsValidEndpointId(old_s, Bits(ids, 15, 0)))
       && IsValidMessageFlags(old_s, flags)
       && CalleeCanHandleRequest(old_s, Bits(ids, 15, 0))
       && CallerAllowedToInvoke(old_s, Bits(ids, 31, 16), 0x84000070)
       && SupportsDirectResponseReceipt(old_s, Bits(ids, 15, 0))
       && IsImplementedAtFfaInstance(old_s, 0x84000070)
       && !ReceiverEncounteredUnexpectedError(old_s, Bits(ids, 15, 0)))
    ==> result == FFA_SUCCESS
  && (result != FFA_SUCCESS)
    ==> EndpointAt(new_s, Bits(ids, 15, 0)).state == EndpointAt(old_s, Bits(ids, 15, 0)).state
  && (result != FFA_SUCCESS)
    ==> EndpointAt(new_s, Bits(ids, 15, 0)).direct_msg == EndpointAt(old_s, Bits(ids, 15, 0)).direct_msg
  && (result != FFA_SUCCESS)
    ==> EndpointAt(new_s, Bits(ids, 31, 16)).state == EndpointAt(old_s, Bits(ids, 31, 16)).state
  && (!( (IsValidEndpointId(old_s, Bits(ids, 31, 16)) || !IsValidEndpointId(old_s, Bits(ids, 15, 0))) )
       ==> result == FFA_SUCCESS)
}