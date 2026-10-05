pub open spec fn ffa_el3_intr_handle_spec(result: UInt32, ffa_instance: UInt32, scr_fiq: bool, old_s: S, new_s: S) -> bool {
    (!IsSupportedFfaInstance(ffa_instance, old_s.conduit) ==> ResultEqual(result, FFA_ERROR_NOT_SUPPORTED))
    && ((old_s.caller_el == S_EL1 || old_s.caller_el == S_EL2) && scr_fiq ==> ResultEqual(result, FFA_ERROR_NOT_SUPPORTED))
    && (ResultEqual(result, FFA_SUCCESS) ==> PendingGroup0InterruptHandledByEl3())
    && (ResultEqual(result, FFA_SUCCESS) ==> (!El3SwitchedSecurityState() && ReturnsToCallingElInSecureState()))
}