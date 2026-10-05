pub open spec fn ffa_normal_world_resume_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (IsSecurePhysicalInstance(old_s.instance) && !NormalWorldWasPreempted(old_s.current_pe) ==> ResultEqual(result, DENIED))
    && (!IsSecurePhysicalInstance(old_s.instance) ==> ResultEqual(result, NOT_SUPPORTED))
    && (result != DENIED && result != NOT_SUPPORTED ==> NormalWorldExecutionResumed(new_s.current_pe))
    && (result != DENIED && result != NOT_SUPPORTED ==> NormalWorldPeState(new_s.current_pe) == SavedPreemptedPeState(old_s.current_pe))
    && (result != DENIED && result != NOT_SUPPORTED ==> CompletedByFfaFunctionInvocation(new_s, ERET))
}