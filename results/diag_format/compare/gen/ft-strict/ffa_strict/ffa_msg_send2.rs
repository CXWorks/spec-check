pub open spec fn ffa_msg_send2_spec(sender_vm_id: UInt32, flags: UInt32, result: Result<(), FfaCommandReturnCode>, error_code: Int32, old_s: S, new_s: S) -> bool {
  (!IsImplementedAtInstance(old_s, FFA_MSG_SEND2) ==> ResultEqual(result, NOT_SUPPORTED))
  && ((IsVirtualInstance(old_s) || IsSecurePhysicalInstance(old_s)) && sender_vm_id != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (IsNonSecurePhysicalInstance(old_s) && !IsValidSenderVmId(old_s, Bits(sender_vm_id, 31, 16)) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsValidSenderId(old_s, MsgSender(old_s, caller, sender_vm_id)) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsValidReceiverId(old_s, MsgReceiver(old_s, caller, sender_vm_id)) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (MsgOffset(old_s, caller, sender_vm_id) < PartitionMsgHeaderSize(old_s) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!MsgPayloadFitsInTxBuffer(old_s, caller, sender_vm_id) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsRecognizedUuid(old_s, MsgUuid(old_s, caller, sender_vm_id)) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!RxBufferIsFree(old_s, MsgReceiver(old_s, caller, sender_vm_id)) ==> ResultEqual(result, BUSY))
  && (!CalleeCanHandleRequest(old_s) ==> ResultEqual(result, DENIED))
  && (!CallerMayInvoke(old_s, caller, FFA_MSG_SEND2) ==> ResultEqual(result, DENIED))
  && (!SupportsIndirectMessaging(old_s, MsgReceiver(old_s, caller, sender_vm_id)) ==> ResultEqual(result, DENIED))
  && (!RxBufferHasSpaceForMsg(old_s, MsgReceiver(old_s, caller, sender_vm_id), caller, sender_vm_id) ==> ResultEqual(result, NO_MEMORY))
  && (result == FFA_SUCCESS ==> ResultEqual(result, FFA_SUCCESS))
  && (result == FFA_SUCCESS ==> RxBufferHoldsMsg(new_s, MsgReceiver(new_s, caller, sender_vm_id), SourceTxBuffer(new_s, caller, sender_vm_id)))
  && (result == FFA_SUCCESS ==> ReceiverSchedulerNotifiedToRun(new_s, MsgReceiver(new_s, caller, sender_vm_id)))
  && ((!(IsImplementedAtInstance(old_s, FFA_MSG_SEND2)) &&
       !(((IsVirtualInstance(old_s) || IsSecurePhysicalInstance(old_s)) && sender_vm_id != 0)) &&
       !(IsNonSecurePhysicalInstance(old_s) && !IsValidSenderVmId(old_s, Bits(sender_vm_id, 31, 16))) &&
       IsValidSenderId(old_s, MsgSender(old_s, caller, sender_vm_id)) &&
       IsValidReceiverId(old_s, MsgReceiver(old_s, caller, sender_vm_id)) &&
       !(MsgOffset(old_s, caller, sender_vm_id) < PartitionMsgHeaderSize(old_s)) &&
       MsgPayloadFitsInTxBuffer(old_s, caller, sender_vm_id) &&
       IsRecognizedUuid(old_s, MsgUuid(old_s, caller, sender_vm_id)) &&
       RxBufferIsFree(old_s, MsgReceiver(old_s, caller, sender_vm_id)) &&
       CalleeCanHandleRequest(old_s) &&
       CallerMayInvoke(old_s, caller, FFA_MSG_SEND2) &&
       SupportsIndirectMessaging(old_s, MsgReceiver(old_s, caller, sender_vm_id)) &&
       RxBufferHasSpaceForMsg(old_s, MsgReceiver(old_s, caller, sender_vm_id), caller, sender_vm_id))
    ==> result == FFA_SUCCESS)
  && (result != FFA_SUCCESS
    ==> RxBufferHoldsMsg(new_s, MsgReceiver(new_s, caller, sender_vm_id), SourceTxBuffer(new_s, caller, sender_vm_id)) == RxBufferHoldsMsg(old_s, MsgReceiver(old_s, caller, sender_vm_id), SourceTxBuffer(old_s, caller, sender_vm_id)))
  && (result != FFA_SUCCESS
    ==> ReceiverSchedulerNotifiedToRun(new_s, MsgReceiver(new_s, caller, sender_vm_id)) == ReceiverSchedulerNotifiedToRun(old_s, MsgReceiver(old_s, caller, sender_vm_id)))
}