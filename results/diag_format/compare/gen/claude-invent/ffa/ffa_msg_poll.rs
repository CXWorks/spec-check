pub open spec fn ffa_msg_poll_spec(function_id: UInt32, result: FfaReturn, old_s: S, new_s: S) -> bool {
    (!FfaMsgPollImplemented(old_s) ==> (FfaErrorEqual(result, NOT_SUPPORTED) && new_s == old_s))
    && ((FfaMsgPollImplemented(old_s)
        && (CallerProcessingDirectRequest(old_s) || !CalleeReadyForRequest(old_s)))
        ==> (FfaErrorEqual(result, DENIED) && new_s == old_s))
    && ((FfaMsgPollImplemented(old_s)
        && !CallerProcessingDirectRequest(old_s)
        && CalleeReadyForRequest(old_s)
        && !CallerRxBufferMessageAvailable(old_s))
        ==> (FfaErrorEqual(result, RETRY) && new_s == old_s))
    && ((function_id == 0x8400006Au32
        && FfaMsgPollImplemented(old_s)
        && !CallerProcessingDirectRequest(old_s)
        && CalleeReadyForRequest(old_s)
        && CallerRxBufferMessageAvailable(old_s))
        ==> FfaResultIsMsgSend(result))
}
