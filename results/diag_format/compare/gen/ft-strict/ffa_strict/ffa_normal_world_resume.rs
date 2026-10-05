pub open spec fn ffa_normal_world_resume_spec(ffa_instance: FfaInstance, result: Int32, old_s: S, new_s: S) -> bool {
  (!IsSecurePhysicalInstance(old_s, ffa_instance) ==> ResultEqual(result, NOT_SUPPORTED))
  && (IsSecurePhysicalInstance(old_s, ffa_instance) && !NormalWorldWasPreempted(old_s, current_pe) ==> ResultEqual(result, DENIED))
  && (result == 0 ==> NormalWorldResumed(new_s, current_pe))
  && (result == 0 ==> PeState(new_s, current_pe) == SavedNormalWorldPeState(new_s, current_pe))
  && (result == 0 ==> CompletedByFfaFunctionInvocationViaEret(new_s, current_pe))
  && ((IsSecurePhysicalInstance(old_s, ffa_instance) && NormalWorldWasPreempted(old_s, current_pe))
    ==> result == 0)
}