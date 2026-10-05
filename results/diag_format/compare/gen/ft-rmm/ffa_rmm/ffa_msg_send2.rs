pub open spec fn ffa_msg_send2_spec(sender_vm_id: UInt16, flags: Bits1, reserved: UInt32, result: Result<(), FfaStatusCode>, old_s: S, new_s: S) -> bool {
  (!IsNonSecurePhysicalInstance(old_s) && sender_vm_id != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (IsNonSecurePhysicalInstance(old_s) && !IsValidSenderVmId(old_s, sender_vm_id) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsValidEndpointId(old_s, MsgHeader(old_s).sender_id) || !IsValidEndpointId(old_s, MsgHeader(old_s).receiver_id) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (MsgHeader(old_s).offset < MsgHeaderSize(old_s) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!MsgPayloadFitsInTxBuffer(old_s, MsgHeader(old_s)) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsRecognizedUuid(old_s, MsgHeader(old_s).uuid) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!RxBufferIsFree(old_s, MsgHeader(old_s).receiver_id) ==> ResultEqual(result, BUSY))
  && (!CalleeCanHandleRequest(old_s) ==> ResultEqual(result, DENIED))
  && (!CallerMayInvoke(old_s, FFA_MSG_SEND2) ==> ResultEqual(result, DENIED))
  && (!SupportsIndirectMessaging(old_s, MsgHeader(old_s).receiver_id) ==> ResultEqual(result, DENIED))
  && (!RxBufferHasSpaceFor(old_s, MsgHeader(old_s).receiver_id, MsgHeader(old_s)) ==> ResultEqual(result, NO_MEMORY))
  && (!IsImplementedAtInstance(old_s, FFA_MSG_SEND2) ==> ResultEqual(result, NOT_SUPPORTED))
  && (result == FFA_SUCCESS ==> ResultEqual(result, FFA_SUCCESS))
  && (result == FFA_SUCCESS ==> RxBufferContains(new_s, MsgHeader(new_s).receiver_id, PartitionMessage(SenderTxBuffer(new_s))))
  && (result == FFA_SUCCESS ==> RxBufferFullNotified(new_s, MsgHeader(new_s).receiver_id))
  && ((!(IsNonSecurePhysicalInstance(old_s) && sender_vm_id != 0) &&
       !(IsNonSecurePhysicalInstance(old_s) && !IsValidSenderVmId(old_s, sender_vm_id)) &&
       !(!IsValidEndpointId(old_s, MsgHeader(old_s).sender_id) || !IsValidEndpointId(old_s, MsgHeader(old_s).receiver_id)) &&
       !(MsgHeader(old_s).offset < MsgHeaderSize(old_s)) &&
       MsgPayloadFitsInTxBuffer(old_s, MsgHeader(old_s)) &&
       IsRecognizedUuid(old_s, MsgHeader(old_s).uuid) &&
       RxBufferIsFree(old_s, MsgHeader(old_s).receiver_id) &&
       CalleeCanHandleRequest(old_s) &&
       CallerMayInvoke(old_s, FFA_MSG_SEND2) &&
       SupportsIndirectMessaging(old_s, MsgHeader(old_s).receiver_id) &&
       RxBufferHasSpaceFor(old_s, MsgHeader(old_s).receiver_id, MsgHeader(old_s)) &&
       IsImplementedAtInstance(old_s, FFA_MSG_SEND2))
    ==> result == FFA_SUCCESS)
  && (result != FFA_SUCCESS
    ==> RxBuffer(new_s, MsgHeader(new_s).receiver_id) == RxBuffer(old_s, MsgHeader(old_s).receiver_id))
  && (result != FFA_SUCCESS
    ==> RxBufferFullNotification(new_s, MsgHeader(new_s).receiver_id) == RxBufferFullNotification(old_s, MsgHeader(old_s).receiver_id))
}