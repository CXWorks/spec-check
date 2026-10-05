pub open spec fn ffa_run_spec(target_info: UInt32, result: UInt32, error_code: Int32, old_s: S, new_s: S) -> bool {
  (!IsImplementedAtInstance(old_s, FFA_RUN, ffa_instance) ==> ResultEqual(result, FFA_ERROR)) && ResultEqual(error_code, NOT_SUPPORTED)
  && (!IsRecognizedEndpointId(old_s, Bits(target_info, 31, 16)) ==> ResultEqual(result, FFA_ERROR)) && ResultEqual(error_code, INVALID_PARAMETERS)
  && (!IsRecognizedVcpuId(old_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0)) ==> ResultEqual(result, FFA_ERROR)) && ResultEqual(error_code, INVALID_PARAMETERS)
  && (IsVcpuPinnedToOtherPe(old_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0), CurrentPe()) ==> ResultEqual(result, FFA_ERROR)) && ResultEqual(error_code, INVALID_PARAMETERS)
  && (!CalleeCanHandleRequest(old_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0)) ==> ResultEqual(result, FFA_ERROR)) && ResultEqual(error_code, DENIED)
  && (!CallerAllowedToInvokeRun(old_s, caller, CpuCycleAllocationMode()) ==> ResultEqual(result, FFA_ERROR)) && ResultEqual(error_code, DENIED)
  && (IsVcpuBusy(old_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0)) ==> ResultEqual(result, FFA_ERROR)) && ResultEqual(error_code, BUSY)
  && (HasVcpuOrVmAborted(old_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0)) ==> ResultEqual(result, FFA_ERROR)) && ResultEqual(error_code, ABORTED)
  && (!IsReceiverReady(old_s, Bits(target_info, 31, 16)) ==> ResultEqual(result, FFA_ERROR)) && ResultEqual(error_code, NOT_READY)
  && (result == FFA_INTERRUPT || result == FFA_MSG_WAIT || result == FFA_YIELD || result == FFA_MSG_SEND_DIRECT_RESP ==> true)
  && (PreExecCtxState(old_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0)) == WAITING ==> TransitionedToRunning(new_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0)))
  && (PreExecCtxState(old_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0)) == BLOCKED ==> TransitionedToRunning(new_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0)))
  && (PreExecCtxState(old_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0)) == PREEMPTED ==> TransitionedToRunningViaEret(new_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0)))
  && (IsPhysicalInstance(old_s, ffa_instance) ==> RequestedTransitionFromPartitionManager(new_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0)))
  && ((IsVirtualInstance(old_s, ffa_instance) && (conduit == SMC || conduit == HVC || conduit == SVC)) ==> CallerWasBlockedDuringRun(new_s, caller))
  && (((IsVirtualInstance(old_s, ffa_instance) && (conduit == SMC || conduit == HVC || conduit == SVC)) || (ffa_instance == NS_PHYSICAL && conduit == SMC)) ==> (ExecCtxState(new_s, caller) == RUNNING && CompletedViaEret(result)))
  && ((ffa_instance == SECURE_PHYSICAL && conduit == ERET) ==> CompletedViaSmc(result))
  && ((IsImplementedAtInstance(old_s, FFA_RUN, ffa_instance) &&
       IsRecognizedEndpointId(old_s, Bits(target_info, 31, 16)) &&
       IsRecognizedVcpuId(old_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0)) &&
       !IsVcpuPinnedToOtherPe(old_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0), CurrentPe()) &&
       CalleeCanHandleRequest(old_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0)) &&
       CallerAllowedToInvokeRun(old_s, caller, CpuCycleAllocationMode()) &&
       !IsVcpuBusy(old_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0)) &&
       !HasVcpuOrVmAborted(old_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0)) &&
       IsReceiverReady(old_s, Bits(target_info, 31, 16)))
    ==> result == FFA_INTERRUPT || result == FFA_MSG_WAIT || result == FFA_YIELD || result == FFA_MSG_SEND_DIRECT_RESP)
  && ((!(IsImplementedAtInstance(old_s, FFA_RUN, ffa_instance)) ||
       IsRecognizedEndpointId(old_s, Bits(target_info, 31, 16)) ||
       !(IsRecognizedVcpuId(old_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0))) ||
       IsVcpuPinnedToOtherPe(old_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0), CurrentPe()) ||
       CalleeCanHandleRequest(old_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0)) ||
       CallerAllowedToInvokeRun(old_s, caller, CpuCycleAllocationMode()) ||
       IsVcpuBusy(old_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0)) ||
       HasVcpuOrVmAborted(old_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0)) ||
       IsReceiverReady(old_s, Bits(target_info, 31, 16)))
    ==> result == FFA_ERROR)
  && (result != FFA_INTERRUPT && result != FFA_MSG_WAIT && result != FFA_YIELD && result != FFA_MSG_SEND_DIRECT_RESP
    ==> PreExecCtxState(new_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0)) == PreExecCtxState(old_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0)))
  && (result != FFA_INTERRUPT && result != FFA_MSG_WAIT && result != FFA_YIELD && result != FFA_MSG_SEND_DIRECT_RESP
    ==> PreExecCtxState(new_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0)) == PreExecCtxState(old_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0)))
  && (result != FFA_INTERRUPT && result != FFA_MSG_WAIT && result != FFA_YIELD && result != FFA_MSG_SEND_DIRECT_RESP
    ==> PreExecCtxState(new_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0)) == PreExecCtxState(old_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0)))
  && (result != FFA_INTERRUPT && result != FFA_MSG_WAIT && result != FFA_YIELD && result != FFA_MSG_SEND_DIRECT_RESP
    ==> PreExecCtxState(new_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0)) == PreExecCtxState(old_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0)))
  && (!(IsImplementedAtInstance(old_s, FFA_RUN, ffa_instance) &&
       IsRecognizedEndpointId(old_s, Bits(target_info, 31, 16)) &&
       IsRecognizedVcpuId(old_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0)) &&
       !IsVcpuPinnedToOtherPe(old_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0), CurrentPe()) &&
       CalleeCanHandleRequest(old_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0)) &&
       CallerAllowedToInvokeRun(old_s, caller, CpuCycleAllocationMode()) &&
       !IsVcpuBusy(old_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0)) &&
       !HasVcpuOrVmAborted(old_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0)) &&
       IsReceiverReady(old_s, Bits(target_info, 31, 16)))
    ==> result == FFA_ERROR)
  && (result == FFA_ERROR
    ==> PreExecCtxState(new_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0)) == PreExecCtxState(old_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0)))
  && (result == FFA_ERROR
    ==> PreExecCtxState(new_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0)) == PreExecCtxState(old_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0)))
  && (result == FFA_ERROR
    ==> PreExecCtxState(new_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0)) == PreExecCtxState(old_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0)))
  && (result == FFA_ERROR
    ==> PreExecCtxState(new_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0)) == PreExecCtxState(old_s, Bits(target_info, 31, 16), Bits(target_info, 15, 0)))
}