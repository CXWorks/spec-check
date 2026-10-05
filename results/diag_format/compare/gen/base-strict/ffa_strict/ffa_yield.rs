pub open spec fn ffa_yield_spec(result: u32, status: i32, old_s: S, new_s: S) -> bool {
    (ConduitIsEret() && (!IsRecognizedEndpointId(Bits(old_s.cmd_input_ids, 31, 16)) || !IsRecognizedVcpuId(Bits(old_s.cmd_input_ids, 31, 16), Bits(old_s.cmd_input_ids, 15, 0))) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!CalleeCanHandleRequest() ==> ResultEqual(result, DENIED))
    && (!IsImplementedAtInstance(FFA_YIELD) ==> ResultEqual(result, NOT_SUPPORTED))
    && (ExecutionYieldedToScheduler(CallerContext(old_s)))
    && (ExecutionContextState(CallerContext(old_s)) == RUNNING)
    && (ResultEqual(result, FFA_RUN) || ResultEqual(result, FFA_INTERRUPT))
    && (IsSEL0Endpoint(CallerContext(old_s)) ==> ResultEqual(result, FFA_RUN))
    && ((IsVmVcpu(CallerContext(old_s)) && Timeout64(old_s.cmd_input_timeout_hi, old_s.cmd_input_timeout_lo) != 0) ==> VcpuRunAfterTimeout(Bits(old_s.cmd_input_ids, 31, 16), Bits(old_s.cmd_input_ids, 15, 0), Timeout64(old_s.cmd_input_timeout_hi, old_s.cmd_input_timeout_lo)))
}