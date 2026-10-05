pub open spec fn ffa_yield_spec(endpoint_id: UInt16, vcpu_id: UInt16, timeout_lo: UInt32, timeout_hi: UInt32, result: Result<(), FfaStatusCode>, status: Int32, old_s: S, new_s: S) -> bool {
  (!IsFfaYieldImplemented(old_s, ffa_instance) ==> ResultEqual(result, NOT_SUPPORTED))
  && (conduit == ERET && !IsValidEndpointVcpuId(old_s, endpoint_id, vcpu_id) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!CalleeCanHandleRequest(old_s, callee) ==> ResultEqual(result, DENIED))
  && (result == FFA_RUN ==> ResultEqual(result, FFA_RUN))
  && (!IsSEl0Endpoint(old_s, caller) ==> ResultEqual(result, FFA_INTERRUPT))
  && (IsVmVcpu(old_s, caller) && TimeoutSpecified(old_s, timeout_hi, timeout_lo) ==> VcpuScheduledAfter(new_s, endpoint_id, vcpu_id, timeout_hi:timeout_lo))
  && ((result == FFA_RUN || result == FFA_INTERRUPT) ==> CallerContext(new_s).state == RUNNING)
  && (result != FFA_ERROR ==> CallerContext(new_s).state == RUNNING)
  && ((!(IsFfaYieldImplemented(old_s, ffa_instance)) &&
       !(conduit == ERET && !IsValidEndpointVcpuId(old_s, endpoint_id, vcpu_id)) &&
       CalleeCanHandleRequest(old_s, callee))
    ==> ResultEqual(result, FFA_RUN))
  && (result == FFA_ERROR ==> CallerContext(new_s).state == CallerContext(old_s).state)
  && (result != FFA_ERROR && result != FFA_INTERRUPT ==> CallerContext(new_s).state == CallerContext(old_s).state)
  && (result == FFA_INTERRUPT ==> CallerContext(new_s).state == CallerContext(old_s).state)
}