pub open spec fn ffa_el3_intr_handle_spec(result: UInt32, error_code: Int32, old_s: S, new_s: S) -> bool {
    (!IsSupportedFfaInstance(old_s.current_instance) ==> ResultEqual(result, FFA_ERROR) && error_code == NOT_SUPPORTED)
    && ((old_s.CallerEl() == S_EL1 || old_s.CallerEl() == S_EL2) && old_s.SCR_EL3.FIQ == 1 ==> ResultEqual(result, FFA_ERROR) && error_code == NOT_SUPPORTED)
    && (ResultEqual(result, FFA_SUCCESS) ==> PendingInterruptHandledByEl3Firmware() && new_s.SecurityStateOnReturn() == Secure && new_s.ReturnEl() == old_s.CallerEl())
}