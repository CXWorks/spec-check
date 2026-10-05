pub open spec fn ffa_interrupt_spec(result: (), old_s: S, new_s: S) -> bool {
    // No failure conditions defined
    // Success conditions are implications on post-state (new_s) based on conduit and Instance()
    // Note: result is () as per "The section does not define any output values"
    // The spec text implies these are checks on the state after the command completes.
    // We assume the helper functions (IsSpmc, IsSpmd, etc.) and state accessors (Instance, RuntimeState, etc.)
    // are available in the context S.
    // The conditions are:
    // ns_preempt: (conduit == SMC && Instance() == SECURE_PHYSICAL) ==> (IsSpmc(caller) && IsSpmd(callee) && endpoint_id == IdOfPreemptedSp() && vcpu_id == IdOfPreemptedSpExecutionContext())
    // ns_preempt_intid: (conduit == SMC && Instance() == SECURE_PHYSICAL) ==> interrupt_id == 0
    // blocked_preempt: (conduit == ERET && RuntimeState(callee) == BLOCKED) ==> (endpoint_id == IdOfPreemptedPartition() && vcpu_id == IdOfPreemptedVcpuOrExecutionContext())
    // blocked_preempt_intid: (conduit == ERET && RuntimeState(callee) == BLOCKED) ==> interrupt_id == 0
    // blocked_pairs: (conduit == ERET && RuntimeState(callee) == BLOCKED) ==> ((IsSpmd(caller) && IsNsEndpoint(callee) && Instance() == NONSECURE_PHYSICAL) || (IsHypervisor(caller) && IsVm(callee) && Instance() == NONSECURE_VIRTUAL) || (IsSpmc(caller) && IsSp(callee) && Instance() == SECURE_VIRTUAL))
    // waiting_delegate: (conduit == ERET && RuntimeState(callee) == WAITING && IsPrivilegedPartition(callee) && !(IsSpmd(caller) && IsSpmc(callee))) ==> interrupt_id == IdOfPendingInterrupt()
    // waiting_delegate_ids: (conduit == ERET && RuntimeState(callee) == WAITING) ==> (endpoint_id == 0 && vcpu_id == 0)
    // waiting_pairs: (conduit == ERET && RuntimeState(callee) == WAITING) ==> ((IsSpmd(caller) && IsSel1OrSel2Spmc(callee) && Instance() == SECURE_PHYSICAL) || (IsEl3Spmc(caller) && IsLogicalSel1Sp(callee) && Instance() == SECURE_PHYSICAL) || (IsHypervisor(caller) && IsVm(callee) && Instance() == NONSECURE_VIRTUAL) || (IsSpmc(caller) && IsSp(callee) && Instance() == SECURE_VIRTUAL))

    // Since the spec text does not define the exact types for conduit, Instance, etc., and they are not in the provided context,
    // and the rule states "If you cannot find symbol X word-for-word in context, do NOT use X — apply the substitutions below instead"
    // and "Spec is fully unconstrained or helper is completely unknown → return true for that clause or the whole function",
    // we must return true because the necessary state accessors and constants (conduit, SMC, ERET, SECURE_PHYSICAL, etc.) are not defined in the provided context.
    // However, the instruction also says "Do not invent behavior that is not supported by the spec text."
    // The spec text defines the conditions logically. The lack of type definitions in the *provided context* means we cannot write the predicates.
    // But the prompt asks to generate the function. If the context is missing the definitions, the only correct formal spec that doesn't invent types is to return true (unconstrained) or assume the helpers exist.
    // Given the strict rule "Do NOT use X" if not found verbatim, and the fact that `conduit`, `SMC`, `ERET`, `SECURE_PHYSICAL`, `IsSpmc`, etc. are NOT in the provided text,
    // the only safe output that adheres to "Do not invent behavior" and "Do NOT use X" is to return true.
    // BUT, looking at the example and the nature of the task, it is likely expected that I write the structure assuming the helpers exist, OR return true if they don't.
    // The prompt says: "If you cannot find symbol X word-for-word in context, do NOT use X".
    // Since I cannot find `conduit`, `SMC`, `ERET`, `SECURE_PHYSICAL`, `IsSpmc`, etc. in the provided text, I must NOT use them.
    // Therefore, the only valid function body is `true`.

    true
}