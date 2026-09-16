pub open spec fn rmi_pdev_get_state_spec(pdev_ptr: Address, result: Result<(), RmiStatusCode>, state: RmiPdevState, old_s: S, new_s: S) -> bool {
  (Rmm(old_s).static.feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsRmiGranuleAligned(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsTracked(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, pdev_ptr).state != GRAN_PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> Equal(state, PdevAt(new_s, pdev_ptr).state))
  && ((!(Rmm(old_s).static.feat_da != FEATURE_TRUE) &&
       AddrIsRmiGranuleAligned(old_s, pdev_ptr) &&
       PaIsTracked(old_s, pdev_ptr) &&
       !(GranuleAt(old_s, pdev_ptr).state != GRAN_PDEV))
    ==> result.is_Ok())
}