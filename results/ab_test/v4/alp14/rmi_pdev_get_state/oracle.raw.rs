pub open spec fn rmi_pdev_get_state_spec(pdev_ptr: Address, result: Result<(), RmiStatusCode>, state: RmiPdevState, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsGranuleAligned(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> Equal(state, PdevAt(new_s, pdev_ptr).state))
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       AddrIsGranuleAligned(old_s, pdev_ptr) &&
       PaIsDelegable(old_s, pdev_ptr) &&
       !(GranuleAt(old_s, pdev_ptr).state != PDEV))
    ==> result.is_Ok())
}