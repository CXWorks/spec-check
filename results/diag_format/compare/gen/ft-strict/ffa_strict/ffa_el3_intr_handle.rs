pub open spec fn ffa_el3_intr_handle_spec(ffa_instance: FfaInstance, conduit: Conduit, result: UInt32, error_code: Int32, old_s: S, new_s: S) -> bool {
  (!IsSupportedFfaInstance(old_s, ffa_instance, conduit) ==> ResultEqual(result, NOT_SUPPORTED))
  && ((CallerEl == S_EL1 || CallerEl == S_EL2) && SCR_EL3.FIQ == 1 ==> ResultEqual(result, NOT_SUPPORTED))
  && (result == FFA_SUCCESS ==> ResultEqual(result, FFA_SUCCESS))
  && (result == FFA_SUCCESS ==> PendingGroup0InterruptHandledByEl3(new_s))
  && (result == FFA_SUCCESS ==> !El3SwitchedSecurityState(new_s) && ReturnsToCallingElInSecureState(new_s))
  && ((IsSupportedFfaInstance(old_s, ffa_instance, conduit) &&
       !((CallerEl == S_EL1 || CallerEl == S_EL2) && SCR_EL3.FIQ == 1))
    ==> result == FFA_SUCCESS)
  && (result != FFA_SUCCESS
    ==> !PendingGroup0InterruptHandledByEl3(new_s))
  && (result != FFA_SUCCESS
    ==> El3SwitchedSecurityState(new_s))
}