pub open spec fn rmi_dpt_l0_create_spec(l0dptsz: RmiL0DptSize, old_s: S, new_s: S) -> bool {
  (Rmm(old_s).static.feat_ats != FEATURE_TRUE ==> result.status == RMI_ERROR_NOT_SUPPORTED)
  && (l0dpt.state == DPT_L0_VALID ==> result.status == RMI_ERROR_INPUT)
  && (!DptL0SizeIsValid(old_s, l0dptsz) ==> result.status == RMI_ERROR_INPUT)
  && (result.status == RMI_SUCCESS ==> result.status == RMI_SUCCESS)
  && (result.status == RMI_SUCCESS ==> l0dpt.state == DPT_L0_VALID)
  && (result.status == RMI_SUCCESS ==> Rmm(new_s).static.l0dptsz == L0DptSizeFromRmi(new_s, l0dptsz))
  && ((!(Rmm(old_s).static.feat_ats != FEATURE_TRUE) &&
       !(l0dpt.state == DPT_L0_VALID) &&
       DptL0SizeIsValid(old_s, l0dptsz))
    ==> result.status == RMI_SUCCESS)
  && (result.status != RMI_SUCCESS
    ==> l0dpt.state == DPT_L0_INVALID)
  && (result.status != RMI_SUCCESS
    ==> Rmm(new_s).static.l0dptsz == Rmm(old_s).static.l0dptsz)
}