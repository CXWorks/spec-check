pub open spec fn ffa_msg_send2_spec(result: Result<(), FfaErrorCode>, old_s: S, new_s: S, sender_vm_id: UInt32, flags: UInt32) -> bool {
    let vm_id = ((sender_vm_id >> 16u32) & 0xFFFFu32);
    let receiver = FfaMsgHeaderReceiverId(old_s, vm_id);
    let delay_sri = !FfaConduitIsSvc(old_s) && ((flags & 0x2u32) != 0);
    let invalid_params =
        ((FfaIsVirtualInstance(old_s) || FfaIsSecurePhysicalInstance(old_s)) && sender_vm_id != 0)
        || (FfaIsNonSecurePhysicalInstance(old_s) && !FfaIsValidSenderVmId(old_s, vm_id))
        || !FfaMsgHeaderSenderIdValid(old_s, vm_id)
        || !FfaMsgHeaderReceiverIdValid(old_s, vm_id)
        || (FfaMsgHeaderOffset(old_s, vm_id) as int) < (FfaMsgHeaderSize(old_s, vm_id) as int)
        || !FfaMsgPayloadFitsTxBuffer(old_s, vm_id)
        || !FfaMsgHeaderUuidRecognized(old_s, vm_id);
    let busy = !FfaRxBufferFree(old_s, receiver);
    let denied =
        !FfaCalleeCanHandleRequest(old_s)
        || !FfaCallerAllowedMsgSend2(old_s)
        || !FfaReceiverSupportsIndirectMessaging(old_s, receiver);
    let no_memory = !FfaRxBufferHasSpaceForMsg(old_s, receiver, vm_id);
    let not_supported = !FfaMsgSend2Implemented(old_s);
    (not_supported ==> ResultEqual(result, NOT_SUPPORTED))
    && (invalid_params ==> ResultEqual(result, INVALID_PARAMETERS))
    && (busy ==> ResultEqual(result, BUSY))
    && (denied ==> ResultEqual(result, DENIED))
    && (no_memory ==> ResultEqual(result, NO_MEMORY))
    && (result.is_Err() ==> new_s == old_s)
    && ((!not_supported && !invalid_params && !busy && !denied && !no_memory) ==> (
        result.is_Ok()
        && FfaRxBufferContainsMsg(new_s, receiver, FfaTxBufferMsg(old_s, vm_id))
        && FfaRxBufferFullNotificationPending(new_s, receiver)
        && (!delay_sri ==> FfaScheduleReceiverInterruptSignaled(new_s, receiver))
    ))
}
