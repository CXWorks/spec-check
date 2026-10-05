pub open spec fn ffa_msg_send_direct_req2_spec(result: Int32, old_s: S, new_s: S) -> bool {
    let sender_id = (old_s.cmd_input_w1 >> 16) as UInt16;
    let receiver_id = (old_s.cmd_input_w1 & 0xFFFF) as UInt16;
    let uuid_lo = old_s.cmd_input_x2;
    let uuid_hi = old_s.cmd_input_x3;
    let fid = old_s.cmd_input_w0;

    (!IsValidEndpointId(sender_id) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsValidEndpointId(receiver_id) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!AreValidMessageFlags() ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsRecognizedUuid(uuid_lo, uuid_hi) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!CalleeCanHandleRequest() ==> ResultEqual(result, DENIED))
    && (!CallerMayInvoke(FFA_MSG_SEND_DIRECT_REQ2) ==> ResultEqual(result, DENIED))
    && (!EndpointSupportsDirectReq(receiver_id) ==> ResultEqual(result, DENIED))
    && (!IsImplementedAtInstance(FFA_MSG_SEND_DIRECT_REQ2) ==> ResultEqual(result, NOT_SUPPORTED))
    && (EndpointAt(new_s, receiver_id).state == RUNNING ==> ResultEqual(result, BUSY))
    && (EndpointAt(new_s, receiver_id).state == BLOCKED ==> ResultEqual(result, BUSY))
    && (EndpointAt(new_s, receiver_id).state == PREEMPTED ==> ResultEqual(result, BUSY))
    && (EndpointAt(new_s, receiver_id).state == ABORTED ==> ResultEqual(result, ABORTED))
    && (!EndpointIsReady(new_s, receiver_id) ==> ResultEqual(result, NOT_READY))
    && (CalleeInvoked(new_s) == FFA_MSG_SEND_DIRECT_RESP2 ==> DirectRespProvided(new_s))
    && (CalleeInvoked(new_s) == FFA_INTERRUPT ==> DirectRequestInterrupted(new_s))
    && (CalleeInvoked(new_s) == FFA_YIELD ==> (EndpointAt(new_s, receiver_id).state == BLOCKED && EndpointMustBeResumed(new_s, receiver_id)))
    && (CalleeInvoked(new_s) == FFA_SUCCESS ==> (DirectRequestCompletedWithoutResp(new_s) && AllOtherParamsZero(new_s)))
}