pub open spec fn ffa_yield_spec(ids: UInt32, timeout_lo: UInt32, timeout_hi: UInt32, result: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
  (ConduitIsEret(old_s) && (!IsRecognizedEndpointId(old_s, Bits(ids, 31, 16)) || !IsRecognizedVcpuId(old_s, Bits(ids, 31, 16), Bits(ids, 15, 0))) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!CalleeCanHandleRequest(old_s) ==> ResultEqual(result, DENIED))
  && (!IsImplementedAtInstance(old_s, FFA_YIELD) ==> ResultEqual(result, NOT_SUPPORTED))
  && (result == FFA_RUN || result == FFA_INTERRUPT ==> ExecutionYieldedToScheduler(new_s, CallerContext()))
  && (result == FFA_RUN || result == FFA_INTERRUPT ==> ExecutionContextState(new_s, CallerContext()) == RUNNING)
  && (result == FFA_RUN || result == FFA_INTERRUPT ==> IsSEL0Endpoint(new_s, CallerContext()) ==> ResultEqual(result, FFA_RUN))
  && ((IsVmVcpu(new_s, CallerContext()) && Timeout64(timeout_hi, timeout_lo) != 0) ==> VcpuRunAfterTimeout(new_s, Bits(ids, 31, 16), Bits(ids, 15, 0), Timeout64(timeout_hi, timeout_lo)))
  && ((!((ConduitIsEret(old_s) && (!IsRecognizedEndpointId(old_s, Bits(ids, 31, 16)) || !IsRecognizedVcpuId(old_s, Bits(ids, 31, 16), Bits(ids, 15, 0)))) &&
       CalleeCanHandleRequest(old_s) &&
       IsImplementedAtInstance(old_s, FFA_YIELD))
    ==> ResultEqual(result, FFA_RUN) || ResultEqual(result, FFA_INTERRUPT))
  && (result == FFA_ERROR ==> ExecutionContextState(new_s, CallerContext()) == RUNNING)
  && (result != FFA_RUN && result != FFA_INTERRUPT ==> !ExecutionYieldedToScheduler(new_s, CallerContext()))
}