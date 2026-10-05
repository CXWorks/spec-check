pub open spec fn ffa_interrupt_spec(endpoint_id: UInt16, vcpu_id: UInt16, interrupt_id: UInt32, old_s: S, new_s: S) -> bool {
  (instance(new_s) == NS_PHYSICAL ==> conduit(new_s) == ERET)
  && ((instance(new_s) == NS_VIRTUAL || instance(new_s) == S_VIRTUAL) ==> conduit(new_s) == ERET)
  && (instance(new_s) == S_PHYSICAL ==> (conduit(new_s) == SMC || conduit(new_s) == ERET))
  && (conduit(new_s) == SMC ==> (IsSel1OrSel2Spmc(new_s, caller) && IsSpmd(new_s, callee) && instance(new_s) == S_PHYSICAL && IsNsInterruptPreemptingSp(new_s, caller)))
  && (conduit(new_s) == SMC ==> (endpoint_id == PreemptedSpId(new_s, caller) && vcpu_id == PreemptedSpExecutionContextId(new_s, caller)))
  && (conduit(new_s) == SMC ==> interrupt_id == 0)
  && (IsBlockedCalleePreemption(new_s, caller, callee) ==> IsValidBlockedCombination(new_s, caller, callee, instance(new_s)))
  && ((IsBlockedCalleePreemption(new_s, caller, callee) && !IsSel0(new_s, callee)) ==> (endpoint_id == PreemptedPartitionId(new_s, callee) && vcpu_id == PreemptedExecutionContextId(new_s, callee)))
  && (IsBlockedCalleePreemption(new_s, caller, callee) ==> interrupt_id == 0)
  && (IsWaitingCalleeDelegation(new_s, caller, callee) ==> IsValidWaitingCombination(new_s, caller, callee, instance(new_s)))
  && ((IsWaitingCalleeDelegation(new_s, caller, callee) && !IsSel0(new_s, callee) && !(IsSpmd(new_s, caller) && IsSpmc(new_s, callee))) ==> interrupt_id == PendingInterruptId(new_s, callee))
  && (IsWaitingCalleeDelegation(new_s, caller, callee) ==> (endpoint_id == 0 && vcpu_id == 0))
  && (ReservedParameterRegistersZero(new_s))
  && (ControlReturnedToCallee(new_s, caller, callee))
  && ((!(instance(new_s) == NS_PHYSICAL) &&
       !((instance(new_s) == NS_VIRTUAL || instance(new_s) == S_VIRTUAL)) &&
       !(instance(new_s) == S_PHYSICAL) &&
       !(conduit(new_s) == SMC) &&
       !(IsBlockedCalleePreemption(new_s, caller, callee)) &&
       !((IsBlockedCalleePreemption(new_s, caller, callee) && !IsSel0(new_s, callee))) &&
       !(IsWaitingCalleeDelegation(new_s, caller, callee)))
    ==> conduit(new_s) == conduit(old_s))
}