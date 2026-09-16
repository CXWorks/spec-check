pub open spec fn rmi_dpt_l0_create_spec(l0dptsz: RmiL0DptSize, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (Rmm(old_s).static.feat_ats != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (Rmm(old_s).DptL0().state == DPT_L0_VALID ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!DptL0SizeIsValid(old_s, l0dptsz) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> result.get_Ok_0().status == RMI_SUCCESS)
  && (result.is_Ok() ==> Rmm(new_s).DptL0().state == DPT_L0_VALID)
  && (result.is_Ok() ==> Rmm(new_s).static.l0dptsz == L0DptSizeFromRmi(new_s, l0dptsz))
  && ((!(Rmm(old_s).static.feat_ats != FEATURE_TRUE) &&
       !(Rmm(old_s).DptL0().state == DPT_L0_VALID) &&
       DptL0SizeIsValid(old_s, l0dptsz))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> Rmm(new_s).DptL0().state == Rmm(old_s).DptL0().state)
  && (result.is_Err()
    ==> Rmm(new_s).static.l0dptsz == Rmm(old_s).static.l0dptsz)
}