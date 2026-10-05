pub open spec fn ffa_msg_wait_spec(result: Int32, old_s: S, new_s: S) -> bool {
    let instance = Instance(old_s);
    let caller = Caller(old_s);
    let flags = old_s.cmd_input_flags;
    let endpoint_vcpu_ids = old_s.cmd_input_endpoint_vcpu_ids;
    let timeout_lo = old_s.cmd_input_timeout_lo;
    let timeout_hi = old_s.cmd_input_timeout_hi;
    let conduit = Conduit(old_s);

    // Failure conditions
    (
        (IsNsPhysicalInstance(instance) || IsNsVirtualInstance(instance))
        && !IsRecognizedEndpointVcpuId(Bits64(endpoint_vcpu_ids, 31, 16), Bits64(endpoint_vcpu_ids, 15, 0))
        ==> ResultEqual(result, INVALID_PARAMETERS)
    )
    && (
        !CalleeCanHandleRequest(instance)
        ==> ResultEqual(result, DENIED)
    )
    && (
        !IsImplementedAtInstance(FFA_MSG_WAIT, instance)
        ==> ResultEqual(result, NOT_SUPPORTED)
    )

    // Success conditions
    && (
        (IsVirtualInstance(instance) || IsSecurePhysicalInstance(instance))
        ==> (
            ExecutionContextState(new_s, caller) == WAITING
        )
    )
    && (
        (IsVirtualInstance(instance) || IsSecurePhysicalInstance(instance))
        && CallerOwnedRxBufferAtEntry(old_s, caller)
        && Bits32(flags, 0, 0) == 0
        ==> !CallerOwnsRxBuffer(new_s, caller)
    )
    && (
        (IsVirtualInstance(instance) || IsSecurePhysicalInstance(instance))
        && CallerOwnedRxBufferAtEntry(old_s, caller)
        && Bits32(flags, 0, 0) == 1
        ==> CallerOwnsRxBuffer(new_s, caller)
    )
    && (
        IsPhysicalInstance(instance)
        ==> SchedulerInformedOfWaiting(new_s, caller)
    )
    && (
        IsNsVirtualInstance(instance)
        && conduit == ERET
        ==> SchedulerInformedOfWaiting(Bits64(endpoint_vcpu_ids, 31, 16), Bits64(endpoint_vcpu_ids, 15, 0))
    )
    && (
        IsNsVirtualInstance(instance)
        && conduit == ERET
        && (timeout_lo != 0 || timeout_hi != 0)
        ==> VcpuRunAfterTimeout(Bits64(endpoint_vcpu_ids, 31, 16), Bits64(endpoint_vcpu_ids, 15, 0), timeout_hi * 4294967296 + timeout_lo)
    )
    && (
        IsVirtualInstance(instance)
        && (conduit == SMC || conduit == HVC || conduit == SVC)
        ==> CompletesWhenAllocatedCpuCycles(new_s, caller)
    )
    && (
        IsSecurePhysicalInstance(instance)
        ==> CompletesWithAnyFfaAbiInvocation(new_s, caller)
    )
}