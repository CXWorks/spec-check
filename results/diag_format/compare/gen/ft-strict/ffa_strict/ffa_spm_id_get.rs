pub open spec fn ffa_spm_id_get_spec(result: Result<(), RsiCommandReturnCode>, function_id: UInt32, result: UInt32, old_s: S, new_s: S) -> bool {
  (!IsImplementedAtInstance(old_s, FFA_SPM_ID_GET, CallerInstance(old_s)) ==> result == NOT_SUPPORTED)
  && (result == RSI_SUCCESS ==> function_id == RSI_SUCCESS)
  && (result == RSI_SUCCESS ==> Bits(result, 31, 16) == 0)
  && (result == RSI_SUCCESS && (CallerInstance(old_s) == NON_SECURE_PHYSICAL || CallerInstance(old_s) == NON_SECURE_VIRTUAL) ==> Bits(result, 15, 0) == SpmcId(old_s))
  && (result == RSI_SUCCESS && CallerInstance(old_s) == SECURE_VIRTUAL ==> Bits(result, 15, 0) == SpmcId(old_s))
  && (result == RSI_SUCCESS && (CallerInstance(old_s) == SECURE_PHYSICAL && (SpmcExceptionLevel(old_s) == S_EL1 || SpmcExceptionLevel(old_s) == S_EL2)) ==> Bits(result, 15, 0) == SpmdId(old_s))
  && (result == RSI_SUCCESS && CallerInstance(old_s) == SECURE_PHYSICAL && SpmcExceptionLevel(old_s) == EL3 ==> Bits(result, 15, 0) == SpmcId(old_s))
  && ((IsImplementedAtInstance(old_s, FFA_SPM_ID_GET, CallerInstance(old_s)))
    ==> result == RSI_SUCCESS)
}