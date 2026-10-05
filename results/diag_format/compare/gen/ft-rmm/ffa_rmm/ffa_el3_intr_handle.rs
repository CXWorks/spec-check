pub open spec fn ffa_el3_intr_handle_spec(current_instance: CurrentInstance, caller_el: CallerEl, scr_el3_fiq: bool, result: UInt32, error_code: Int32, old_s: S, new_s: S) -> bool {
  (!IsSupportedFfaInstance(old_s, current_instance) ==> ResultEqual(result, FFA_ERROR) && error_code == NOT_SUPPORTED)
  && ((CallerEl(old_s) == S_EL1 || CallerEl(old_s) == S_EL2) && SCR_EL3.FIQ == 1 ==> ResultEqual(result, FFA_ERROR) && error_code == NOT_SUPPORTED)
  && (result == FFA_SUCCESS ==> ResultEqual(result, FFA_SUCCESS))
  && (result == FFA_SUCCESS ==> PendingInterruptHandledByEl3Firmware(new_s))
  && (result == FFA_SUCCESS ==> SecurityStateOnReturn(new_s) == Secure && ReturnEl(new_s) == CallerEl(new_s))
  && ((IsSupportedFfaInstance(old_s, current_instance) &&
       !((CallerEl(old_s) == S_EL1 || CallerEl(old_s) == S_EL2) && SCR_EL3.FIQ == 1))
    ==> ResultEqual(result, FFA_SUCCESS))
  && (result != FFA_SUCCESS
    ==> PendingInterruptHandledByEl3Firmware(new_s))
  && (result != FFA_SUCCESS
    ==> SecurityStateOnReturn(new_s) == Secure && ReturnEl(new_s) == CallerEl(new_s))
}