pub open spec fn ffa_yield_spec(result: u32, old_s: S, new_s: S) -> bool {
    (!IsFfaYieldImplemented(old_s.ffa_instance) ==> ResultEqual(result, NOT_SUPPORTED))
    && (old_s.conduit == ERET && !IsValidEndpointVcpuId(old_s.endpoint_id, old_s.vcpu_id) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!CalleeCanHandleRequest(old_s.callee) ==> ResultEqual(result, DENIED))
    && (CallerContext(new_s).state == BLOCKED ==> CallerContext(old_s).state == RUNNING)
    && (CallerContext(new_s).state == RUNNING ==> CallerContext(old_s).state == BLOCKED)
    && (ResultEqual(result, FFA_RUN) ==> CallerContext(new_s).state == RUNNING)
    && (!IsSEl0Endpoint(old_s.caller) ==> ResultEqual(result, FFA_INTERRUPT))
    && (IsVmVcpu(old_s.caller) && TimeoutSpecified(old_s.timeout_hi, old_s.timeout_lo) ==> VcpuScheduledAfter(old_s.endpoint_id, old_s.vcpu_id, old_s.timeout_hi, old_s.timeout_lo))
}