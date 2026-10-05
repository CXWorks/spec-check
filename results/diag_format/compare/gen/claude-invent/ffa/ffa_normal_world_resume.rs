pub open spec fn ffa_normal_world_resume_spec(result: FfaReturnCode, old_s: S, new_s: S) -> bool {
    (!IsSecurePhysicalFfaInstance(old_s) ==> (result == NOT_SUPPORTED && new_s == old_s))
    && ((IsSecurePhysicalFfaInstance(old_s) && !NormalWorldPreempted(old_s)) ==> (result == DENIED && new_s == old_s))
    && ((IsSecurePhysicalFfaInstance(old_s) && NormalWorldPreempted(old_s)) ==> (
        !IsFfaError(result)
        && NormalWorldResumed(new_s)
        && NormalWorldPeState(new_s) == SavedNormalWorldPeState(old_s)
    ))
}
