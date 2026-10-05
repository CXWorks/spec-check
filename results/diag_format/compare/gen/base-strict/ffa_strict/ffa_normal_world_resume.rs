pub open spec fn ffa_normal_world_resume_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (!IsSecurePhysicalInstance(ffa_instance(old_s)) ==> ResultEqual(result, NOT_SUPPORTED))
    && (IsSecurePhysicalInstance(ffa_instance(old_s)) && !NormalWorldWasPreempted(current_pe(old_s)) ==> ResultEqual(result, DENIED))
    && (result != NOT_SUPPORTED && result != DENIED ==> NormalWorldResumed(current_pe(old_s)))
    && (result != NOT_SUPPORTED && result != DENIED ==> PeState(current_pe(old_s)) == SavedNormalWorldPeState(current_pe(old_s)))
    && (result != NOT_SUPPORTED && result != DENIED ==> CompletedByFfaFunctionInvocationViaEret(current_pe(old_s)))
}