pub open spec fn ffa_msg_wait_spec(error_code: Int32, old_s: S, new_s: S) -> bool {
    // Failure conditions
    ((IsNonSecurePhysicalInstance() || IsNonSecureVirtualInstance()) && !IsRecognizedEndpointOrVcpuId(old_s.ids) ==> error_code == INVALID_PARAMETERS)
    && (!CalleeCanHandleRequest() ==> error_code == DENIED)
    && (!IsImplementedAtInstance(FFA_MSG_WAIT) ==> error_code == NOT_SUPPORTED)
    // Success conditions
    && ((IsVirtualInstance() || IsSecurePhysicalInstance()) && IsValidConduit(old_s) ==> new_s.CallerContext().state == WAITING)
    && ((IsVirtualInstance() || IsSecurePhysicalInstance()) && CallerOwnsRxBuffer(old_s) && (old_s.flags as int) == 0 ==> !CallerOwnsRxBuffer(new_s))
    && ((IsVirtualInstance() || IsSecurePhysicalInstance()) && CallerOwnsRxBuffer(old_s) && (old_s.flags as int) == 1 ==> CallerOwnsRxBuffer(new_s))
    && (IsPhysicalInstance() && IsValidConduit(old_s) ==> SchedulerInformedOfWaitTransition(old_s.CallerContext()))
    && (IsNonSecureVirtualInstance() && Conduit(old_s) == ERET ==> SchedulerInformedOfWaitTransition(old_s.ids))
    && (IsNonSecureVirtualInstance() && Conduit(old_s) == ERET && IsVmVcpu(old_s.ids) && TimeoutSpecified(old_s.timeout_hi, old_s.timeout_lo) ==> SchedulerRunsVcpuAfterTimeout(old_s.ids, old_s.timeout_hi, old_s.timeout_lo))
    && (IsVirtualInstance() && Conduit(old_s) in {SMC, HVC, SVC} ==> CompletesWhenAllocatedCpuCycles(old_s.CallerContext()))
    && (IsSecurePhysicalInstance() ==> CompletesOnInvocationOfAnyFfaAbi())
}