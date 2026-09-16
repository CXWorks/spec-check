pub open spec fn rmi_dpt_l0_destroy_spec(l0dpt: RmmDptL0, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (Rmm(old_s).static.feat_ats != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (l0dpt.state == DPT_L0_INVALID ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (DptL0IsLive(old_s) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> result.get_Err_0() == RMI_SUCCESS)
  && (result.is_Ok() ==> l0dpt.state == DPT_L0_INVALID)
  && ((!(Rmm(old_s).static.feat_ats != FEATURE_TRUE) &&
       !(l0dpt.state == DPT_L0_INVALID) &&
       !(DptL0IsLive(old_s)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> l0dpt.state == l0dpt.state)
}