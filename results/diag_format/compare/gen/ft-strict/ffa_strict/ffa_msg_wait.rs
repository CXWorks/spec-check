pub open spec fn ffa_msg_wait_spec(endpoint_vcpu_ids: UInt32, timeout_lo: UInt32, timeout_hi: UInt32, flags: UInt32, result: Int32, old_s: S, new_s: S) -> bool {
  ((IsNsPhysicalInstance(old_s) || IsNsVirtualInstance(old_s)) && !IsRecognizedEndpointVcpuId(old_s, (endpoint_vcpu_ids >> 16) as int, (endpoint_vcpu_ids) as int) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!CalleeCanHandleRequest(old_s) ==> ResultEqual(result, DENIED))
  && (!IsImplementedAtInstance(old_s, FFA_MSG_WAIT, 0) ==> ResultEqual(result, NOT_SUPPORTED))
  && (result == FFA_SUCCESS && (IsVirtualInstance(old_s) || IsSecurePhysicalInstance(old_s)) ==> ExecutionContextState(new_s, caller) == WAITING)
  && (result == FFA_SUCCESS && (IsVirtualInstance(old_s) || IsSecurePhysicalInstance(old_s)) && CallerOwnedRxBufferAtEntry(old_s, caller) && (flags & 1) == 0 ==> !CallerOwnsRxBuffer(new_s, caller))
  && (result == FFA_SUCCESS && (IsVirtualInstance(old_s) || IsSecurePhysicalInstance(old_s)) && CallerOwnedRxBufferAtEntry(old_s, caller) && (flags & 1) == 1 ==> CallerOwnsRxBuffer(new_s, caller))
  && (result == FFA_SUCCESS && IsPhysicalInstance(old_s) ==> SchedulerInformedOfWaiting(new_s, caller))
  && (result == FFA_SUCCESS && IsNsVirtualInstance(old_s) && conduit == ERET ==> SchedulerInformedOfWaiting(new_s, (endpoint_vcpu_ids >> 16) as int, (endpoint_vcpu_ids) as int))
  && (result == FFA_SUCCESS && IsNsVirtualInstance(old_s) && conduit == ERET && (timeout_lo != 0 || timeout_hi != 0) ==> VcpuRunAfterTimeout(new_s, (endpoint_vcpu_ids >> 16) as int, (endpoint_vcpu_ids) as int, timeout_hi * 4294967296 + timeout_lo))
  && (result == FFA_SUCCESS && IsVirtualInstance(old_s) && (conduit == SMC || conduit == HVC || conduit == SVC) ==> CompletesWhenAllocatedCpuCycles(new_s, caller))
  && (result == FFA_SUCCESS && IsSecurePhysicalInstance(old_s) ==> CompletesWithAnyFfaAbiInvocation(new_s, caller))
  && ((!( (IsNsPhysicalInstance(old_s) || IsNsVirtualInstance(old_s)) && !IsRecognizedEndpointVcpuId(old_s, (endpoint_vcpu_ids >> 16) as int, (endpoint_vcpu_ids) as int)) &&
       CalleeCanHandleRequest(old_s) &&
       IsImplementedAtInstance(old_s, FFA_MSG_WAIT, 0))
    ==> result == FFA_SUCCESS)
  && (result != FFA_SUCCESS
    ==> ExecutionContextState(new_s, caller) == RUNNING)
  && (result != FFA_SUCCESS
    ==> CallerOwnsRxBuffer(new_s, caller) == CallerOwnsRxBuffer(old_s, caller))
  && (result != FFA_SUCCESS
    ==> !SchedulerInformedOfWaiting(new_s, caller))
  && (!(IsNsVirtualInstance(old_s) && conduit == ERET && (timeout_lo != 0 || timeout_hi != 0))
    ==> !VcpuRunAfterTimeout(new_s, (endpoint_vcpu_ids >> 16) as int, (endpoint_vcpu_ids) as int, timeout_hi * 4294967296 + timeout_lo))
}