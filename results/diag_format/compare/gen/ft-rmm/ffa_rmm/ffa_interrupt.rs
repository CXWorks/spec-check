pub open spec fn ffa_interrupt_spec(endpoint_id: UInt16, vcpu_id: UInt16, interrupt_id: UInt32, old_s: S, new_s: S) -> bool {
  (Conduit() == SMC && Instance() == SECURE_PHYSICAL ==> (IsSpmc(caller) && IsSpmd(callee) && endpoint_id == IdOfPreemptedSp() && vcpu_id == IdOfPreemptedSpExecutionContext()))
  && (Conduit() == SMC && Instance() == SECURE_PHYSICAL ==> interrupt_id == 0)
  && (Conduit() == ERET && RuntimeState(callee) == BLOCKED ==> (endpoint_id == IdOfPreemptedPartition() && vcpu_id == IdOfPreemptedVcpuOrExecutionContext()))
  && (Conduit() == ERET && RuntimeState(callee) == BLOCKED ==> interrupt_id == 0)
  && (Conduit() == ERET && RuntimeState(callee) == BLOCKED ==> ((IsSpmd(caller) && IsNsEndpoint(callee) && Instance() == NONSECURE_PHYSICAL) || (IsHypervisor(caller) && IsVm(callee) && Instance() == NONSECURE_VIRTUAL) || (IsSpmc(caller) && IsSp(callee) && Instance() == SECURE_VIRTUAL)))
  && (Conduit() == ERET && RuntimeState(callee) == WAITING && IsPrivilegedPartition(callee) && !(IsSpmd(caller) && IsSpmc(callee)) ==> interrupt_id == IdOfPendingInterrupt())
  && (Conduit() == ERET && RuntimeState(callee) == WAITING ==> (endpoint_id == 0 && vcpu_id == 0))
  && (Conduit() == ERET && RuntimeState(callee) == WAITING ==> ((IsSpmd(caller) && IsSel1OrSel2Spmc(callee) && Instance() == SECURE_PHYSICAL) || (IsEl3Spmc(caller) && IsLogicalSel1Sp(callee) && Instance() == SECURE_PHYSICAL) || (IsHypervisor(caller) && IsVm(callee) && Instance() == NONSECURE_VIRTUAL) || (IsSpmc(caller) && IsSp(callee) && Instance() == SECURE_VIRTUAL)))
  && ((!(Conduit() == SMC && Instance() == SECURE_PHYSICAL) &&
       !(Conduit() == SMC && Instance() == SECURE_PHYSICAL) &&
       !(Conduit() == ERET && RuntimeState(callee) == BLOCKED) &&
       !(Conduit() == ERET && RuntimeState(callee) == BLOCKED) &&
       !(Conduit() == ERET && RuntimeState(callee) == BLOCKED) &&
       !(Conduit() == ERET && RuntimeState(callee) == WAITING && IsPrivilegedPartition(callee) && !(IsSpmd(caller) && IsSpmc(callee))) &&
       !(Conduit() == ERET && RuntimeState(callee) == WAITING) &&
       !(Conduit() == ERET && RuntimeState(callee) == WAITING))
    ==> true)
}