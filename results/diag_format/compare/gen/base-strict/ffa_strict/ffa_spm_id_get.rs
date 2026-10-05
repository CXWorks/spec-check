pub open spec fn ffa_spm_id_get_spec(result: u32, function_id: u32, old_s: S, new_s: S) -> bool {
    (!IsImplementedAtInstance(FFA_SPM_ID_GET, CallerInstance()) ==> ResultEqual(result, NOT_SUPPORTED))
    && (function_id == FFA_SUCCESS ==> (Bits(result, 31, 16) == 0))
    && ((CallerInstance() == NON_SECURE_PHYSICAL || CallerInstance() == NON_SECURE_VIRTUAL) ==> (Bits(result, 15, 0) == SpmcId()))
    && (CallerInstance() == SECURE_VIRTUAL ==> (Bits(result, 15, 0) == SpmcId()))
    && ((CallerInstance() == SECURE_PHYSICAL && (SpmcExceptionLevel() == S_EL1 || SpmcExceptionLevel() == S_EL2)) ==> (Bits(result, 15, 0) == SpmdId()))
    && ((CallerInstance() == SECURE_PHYSICAL && SpmcExceptionLevel() == EL3) ==> (Bits(result, 15, 0) == SpmcId()))
    && (old_s == new_s)
}