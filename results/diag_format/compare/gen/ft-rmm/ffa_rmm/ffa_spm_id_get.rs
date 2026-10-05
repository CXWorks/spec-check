pub open spec fn ffa_spm_id_get_spec(instance: Instance, result: Result<FFAReturn, FFAStatusCode>, id: UInt16, old_s: S, new_s: S) -> bool {
  (!IsImplementedAtInstance(old_s, FFA_SPM_ID_GET, instance) ==> ResultEqual(result, NOT_SUPPORTED))
  && (result == FFA_SUCCESS ==> ResultEqual(result, FFA_SUCCESS))
  && (result == FFA_SUCCESS ==> id[31:16] == 0)
  && (result == FFA_SUCCESS && instance == NON_SECURE_PHYSICAL || instance == NON_SECURE_VIRTUAL ==> id[15:0] == SpmcId())
  && (result == FFA_SUCCESS && instance == SECURE_VIRTUAL ==> id[15:0] == SpmcId())
  && (result == FFA_SUCCESS && instance == SECURE_PHYSICAL && (SpmcEl() == S_EL1 || SpmcEl() == S_EL2) ==> id[15:0] == SpmdId())
  && (result == FFA_SUCCESS && instance == SECURE_PHYSICAL && SpmcEl() == EL3 ==> id[15:0] == SpmcId())
  && ((IsImplementedAtInstance(old_s, FFA_SPM_ID_GET, instance))
    ==> ResultEqual(result, FFA_SUCCESS))
}