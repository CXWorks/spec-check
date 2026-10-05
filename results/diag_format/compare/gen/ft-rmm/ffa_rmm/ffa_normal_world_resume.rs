pub open spec fn ffa_normal_world_resume_spec(instance: SecurePhysicalInstance, current_pe: CurrentPe, result: Int32, old_s: S, new_s: S) -> bool {
  (IsSecurePhysicalInstance(old_s, instance) && !NormalWorldWasPreempted(old_s, current_pe) ==> ResultEqual(result, DENIED))
  && (!IsSecurePhysicalInstance(old_s, instance) ==> ResultEqual(result, NOT_SUPPORTED))
  && (result == 0 ==> NormalWorldExecutionResumed(new_s, current_pe))
  && (result == 0 ==> NormalWorldPeState(new_s, current_pe) == SavedPreemptedPeState(new_s, current_pe))
  && (result == 0 ==> CompletedByFfaFunctionInvocation(new_s, ERET))
  && ((!(IsSecurePhysicalInstance(old_s, instance) && !NormalWorldWasPreempted(old_s, current_pe)) &&
       IsSecurePhysicalInstance(old_s, instance))
    ==> result != 0)
}