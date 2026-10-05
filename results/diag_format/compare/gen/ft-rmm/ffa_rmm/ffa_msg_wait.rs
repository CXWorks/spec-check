pub open spec fn ffa_msg_wait_spec(ids: Bits32, flags: Bits32, timeout_lo: UInt32, timeout_hi: UInt32, error_code: Int32, old_s: S, new_s: S) -> bool {
  ((IsNonSecurePhysicalInstance(old_s) || IsNonSecureVirtualInstance(old_s)) && !IsRecognizedEndpointOrVcpuId(old_s, ids) ==> ResultEqual(error_code, INVALID_PARAMETERS))
  && (!CalleeCanHandleRequest(old_s) ==> ResultEqual(error_code, DENIED))
  && (!IsImplementedAtInstance(old_s, FFA_MSG_WAIT) ==> ResultEqual(error_code, NOT_SUPPORTED))
  && (result.is_Ok() && (IsVirtualInstance(old_s) || IsSecurePhysicalInstance(old_s)) && IsValidConduit(old_s) ==> CallerContext(new_s).state == WAITING)
  && (result.is_Ok() && (IsVirtualInstance(old_s) || IsSecurePhysicalInstance(old_s)) && CallerOwnsRxBuffer(old_s) && flags[0] == 0 ==> !CallerOwnsRxBuffer(new_s))
  && (result.is_Ok() && (IsVirtualInstance(old_s) || IsSecurePhysicalInstance(old_s)) && CallerOwnsRxBuffer(old_s) && flags[0] == 1 ==> CallerOwnsRxBuffer(new_s))
  && (result.is_Ok() && IsPhysicalInstance(old_s) && IsValidConduit(old_s) ==> SchedulerInformedOfWaitTransition(new_s, CallerContext(new_s)))
  && (result.is_Ok() && IsNonSecureVirtualInstance(old_s) && Conduit(old_s) == ERET ==> SchedulerInformedOfWaitTransition(new_s, ids))
  && (result.is_Ok() && IsNonSecureVirtualInstance(old_s) && Conduit(old_s) == ERET && IsVmVcpu(old_s, ids) && TimeoutSpecified(old_s, timeout_hi, timeout_lo) ==> SchedulerRunsVcpuAfterTimeout(new_s, ids, timeout_hi as int,timeout_lo as int))
  && (result.is_Ok() && IsVirtualInstance(old_s) && Conduit(old_s) in {SMC, HVC, SVC} ==> CompletesWhenAllocatedCpuCycles(new_s, CallerContext(new_s)))
  && (result.is_Ok() && IsSecurePhysicalInstance(old_s) ==> CompletesOnInvocationOfAnyFfaAbi(new_s))
  && ((!( (IsNonSecurePhysicalInstance(old_s) || IsNonSecureVirtualInstance(old_s)) && !IsRecognizedEndpointOrVcpuId(old_s, ids)) &&
       CalleeCanHandleRequest(old_s) &&
       IsImplementedAtInstance(old_s, FFA_MSG_WAIT))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> CallerContext(new_s).state == CallerContext(old_s).state)
  && (result != RSI_SUCCESS
    ==> CallerOwnsRxBuffer(new_s) == CallerOwnsRxBuffer(old_s))
  && (result != RSI_SUCCESS
    ==> SchedulerInformedOfWaitTransition(new_s, CallerContext(new_s)))
  && (result != RSI_SUCCESS
    ==> SchedulerInformedOfWaitTransition(new_s, ids))
  && (result != RSI_SUCCESS
    ==> SchedulerRunsVcpuAfterTimeout(new_s, ids, timeout_hi as int,timeout_lo as int))
  && (result != RSI_SUCCESS
    ==> CompletesWhenAllocatedCpuCycles(new_s, CallerContext(new_s)))
  && (result != RSI_SUCCESS
    ==> CompletesOnInvocationOfAnyFfaAbi(new_s))
  && (SchedVcpuTimeout(new_s, ids) == SchedVcpuTimeout(old_s, ids))
}