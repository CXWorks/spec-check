pub open spec fn ffa_msg_send_direct_req2_spec(
    function_id: UInt32,
    sender_receiver_ids: UInt32,
    uuid_lo: UInt64,
    uuid_hi: UInt64,
    result: FfaReturn,
    old_s: S,
    new_s: S,
) -> bool {
    (!FfaFunctionImplemented(old_s, 0xC400008Du32)
        ==> FfaReturnIsError(result, NOT_SUPPORTED) && new_s == old_s)
    && ((!FfaIsValidEndpointId(old_s, ((sender_receiver_ids >> 16u32) & 0xFFFFu32) as u16)
        || !FfaIsValidEndpointId(old_s, (sender_receiver_ids & 0xFFFFu32) as u16)
        || !FfaIsRecognizedUuid(old_s, (sender_receiver_ids & 0xFFFFu32) as u16, uuid_lo, uuid_hi))
        ==> FfaReturnIsError(result, INVALID_PARAMETERS) && new_s == old_s)
    && ((!FfaCalleeCanHandleRequest(old_s)
        || !FfaCallerAllowedToInvoke(old_s, ((sender_receiver_ids >> 16u32) & 0xFFFFu32) as u16, 0xC400008Du32)
        || !FfaReceiverSupportsDirectReq(old_s, (sender_receiver_ids & 0xFFFFu32) as u16))
        ==> FfaReturnIsError(result, DENIED) && new_s == old_s)
    && ((FfaEndpointIsRunning(old_s, (sender_receiver_ids & 0xFFFFu32) as u16)
        || FfaEndpointIsBlocked(old_s, (sender_receiver_ids & 0xFFFFu32) as u16)
        || FfaEndpointIsPreempted(old_s, (sender_receiver_ids & 0xFFFFu32) as u16))
        ==> FfaReturnIsError(result, BUSY) && new_s == old_s)
    && (FfaEndpointIsAborted(old_s, (sender_receiver_ids & 0xFFFFu32) as u16)
        ==> FfaReturnIsError(result, ABORTED) && new_s == old_s)
    && (!FfaEndpointIsReady(old_s, (sender_receiver_ids & 0xFFFFu32) as u16)
        ==> FfaReturnIsError(result, NOT_READY) && new_s == old_s)
    && ((function_id == 0xC400008Du32
        && FfaValidInstanceConduit(old_s, 0xC400008Du32)
        && FfaFunctionImplemented(old_s, 0xC400008Du32)
        && FfaIsValidEndpointId(old_s, ((sender_receiver_ids >> 16u32) & 0xFFFFu32) as u16)
        && FfaIsValidEndpointId(old_s, (sender_receiver_ids & 0xFFFFu32) as u16)
        && FfaIsRecognizedUuid(old_s, (sender_receiver_ids & 0xFFFFu32) as u16, uuid_lo, uuid_hi)
        && FfaCalleeCanHandleRequest(old_s)
        && FfaCallerAllowedToInvoke(old_s, ((sender_receiver_ids >> 16u32) & 0xFFFFu32) as u16, 0xC400008Du32)
        && FfaReceiverSupportsDirectReq(old_s, (sender_receiver_ids & 0xFFFFu32) as u16)
        && !FfaEndpointIsRunning(old_s, (sender_receiver_ids & 0xFFFFu32) as u16)
        && !FfaEndpointIsBlocked(old_s, (sender_receiver_ids & 0xFFFFu32) as u16)
        && !FfaEndpointIsPreempted(old_s, (sender_receiver_ids & 0xFFFFu32) as u16)
        && !FfaEndpointIsAborted(old_s, (sender_receiver_ids & 0xFFFFu32) as u16)
        && FfaEndpointIsReady(old_s, (sender_receiver_ids & 0xFFFFu32) as u16))
        ==> (FfaReturnFunctionIs(result, FFA_MSG_SEND_DIRECT_RESP2)
            || FfaReturnFunctionIs(result, FFA_INTERRUPT)
            || FfaReturnFunctionIs(result, FFA_YIELD)
            || (FfaReturnFunctionIs(result, FFA_SUCCESS) && FfaReturnOtherParamsZero(result))))
}
