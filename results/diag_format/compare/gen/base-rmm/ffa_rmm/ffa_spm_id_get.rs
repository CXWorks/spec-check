pub open spec fn ffa_spm_id_get_spec(result: Int32, id: UInt16, old_s: S, new_s: S) -> bool {
    (!IsImplementedAtInstance(FFA_SPM_ID_GET, old_s.ffa_instance) ==> ResultEqual(result, NOT_SUPPORTED))
    && (ResultEqual(result, FFA_SUCCESS) ==> id[31:16] == 0)
    && (ResultEqual(result, FFA_SUCCESS) ==> (old_s.ffa_instance == NON_SECURE_PHYSICAL || old_s.ffa_instance == NON_SECURE_VIRTUAL ==> id[15:0] == SpmcId()))
    && (ResultEqual(result, FFA_SUCCESS) ==> (old_s.ffa_instance == SECURE_VIRTUAL ==> id[15:0] == SpmcId()))
    && (ResultEqual(result, FFA_SUCCESS) ==> (old_s.ffa_instance == SECURE_PHYSICAL && (SpmcEl(old_s) == S_EL1 || SpmcEl(old_s) == S_EL2) ==> id[15:0] == SpmdId()))
    && (ResultEqual(result, FFA_SUCCESS) ==> (old_s.ffa_instance == SECURE_PHYSICAL && SpmcEl(old_s) == EL3 ==> id[15:0] == SpmcId()))
    && (ResultEqual(result, FFA_SUCCESS) ==> old_s == new_s)
}