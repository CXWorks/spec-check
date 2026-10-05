pub open spec fn ffa_run_spec(target_id: UInt16, target_vcpu: UInt16, result: Int32, old_s: S, new_s: S) -> bool {
  (!IsValidEndpointId(old_s, target_id) || !IsValidVcpuId(old_s, target_id, target_vcpu) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (IsVcpuPinnedToOtherPe(old_s, target_id, target_vcpu) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsImplementedAtInstance(old_s, FFA_RUN) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsCalleeInStateToHandleRequest(old_s, target_id, target_vcpu) ==> ResultEqual(result, DENIED))
  && (!IsCallerAllowedToInvoke(old_s, FFA_RUN) ==> ResultEqual(result, DENIED))
  && (IsVcpuBusy(old_s, target_id, target_vcpu) ==> ResultEqual(result, BUSY))
  && (HasAborted(old_s, target_id, target_vcpu) ==> ResultEqual(result, ABORTED))
  && (!IsReceiverReady(old_s, target_id) ==> ResultEqual(result, NOT_READY))
  && (result == FFA_INTERRUPT || result == FFA_MSG_WAIT || result == FFA_YIELD || result == FFA_MSG_SEND_DIRECT_RESP ==> ExecContextAt(new_s, target_id, target_vcpu).state == RUNNING)
  && (result == FFA_INTERRUPT || result == FFA_MSG_WAIT || result == FFA_YIELD || result == FFA_MSG_SEND_DIRECT_RESP && ExecContextAt(old_s, target_id, target_vcpu).state == BLOCKED ==> ExecContextAt(new_s, target_id, target_vcpu).state == RUNNING)
  && (result == FFA_INTERRUPT || result == FFA_MSG_WAIT || result == FFA_YIELD || result == FFA_MSG_SEND_DIRECT_RESP && ExecContextAt(old_s, target_id, target_vcpu).state == PREEMPTED ==> ExecContextAt(new_s, target_id, target_vcpu).state == RUNNING)
  && (result == FFA_INTERRUPT || result == FFA_MSG_WAIT || result == FFA_YIELD || result == FFA_MSG_SEND_DIRECT_RESP ==> CallerExecContext(new_s).state == RUNNING)
  && ((IsValidEndpointId(old_s, target_id) && IsValidVcpuId(old_s, target_id, target_vcpu) &&
       !IsVcpuPinnedToOtherPe(old_s, target_id, target_vcpu) &&
       IsImplementedAtInstance(old_s, FFA_RUN) &&
       IsCalleeInStateToHandleRequest(old_s, target_id, target_vcpu) &&
       IsCallerAllowedToInvoke(old_s, FFA_RUN) &&
       !IsVcpuBusy(old_s, target_id, target_vcpu) &&
       !HasAborted(old_s, target_id, target_vcpu) &&
       IsReceiverReady(old_s, target_id))
    ==> result == FFA_INTERRUPT || result == FFA_MSG_WAIT || result == FFA_YIELD || result == FFA_MSG_SEND_DIRECT_RESP)
  && (result == FFA_ERROR
    ==> ExecContextAt(new_s, target_id, target_vcpu).state == ExecContextAt(old_s, target_id, target_vcpu).state)
  && (result == FFA_ERROR
    ==> CallerExecContext(new_s).state == CallerExecContext(old_s).state)
  && (!(result == FFA_INTERRUPT || result == FFA_MSG_WAIT || result == FFA_YIELD || result == FFA_MSG_SEND_DIRECT_RESP)
    ==> ExecContextAt(new_s, target_id, target_vcpu).state == ExecContextAt(old_s, target_id, target_vcpu).state)
  && (!(result == FFA_INTERRUPT || result == FFA_MSG_WAIT || result == FFA_YIELD || result == FFA_MSG_SEND_DIRECT_RESP)
    ==> CallerExecContext(new_s).state == CallerExecContext(old_s).state)
}