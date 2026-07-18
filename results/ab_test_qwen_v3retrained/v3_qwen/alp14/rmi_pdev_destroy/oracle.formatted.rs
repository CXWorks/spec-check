pub open spec fn rmi_pdev_destroy_spec(pdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsGranuleAligned(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (PdevAt(old_s, pdev_ptr).state != PDEV_STOPPED ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (PdevAt(old_s, pdev_ptr).p2p_stream_valid == RMM_TRUE ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (result.is_Ok() ==> GranuleAt(new_s, pdev_ptr).state == DELEGATED)
  && (result.is_Ok() ==> AuxStateEqual32(new_s, PdevAt(new_s, pdev_ptr).aux, PdevAt(new_s, pdev_ptr).num_aux as int, DELEGATED))
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       AddrIsGranuleAligned(old_s, pdev_ptr) &&
       PaIsDelegable(old_s, pdev_ptr) &&
       !(GranuleAt(old_s, pdev_ptr).state != PDEV) &&
       !(PdevAt(old_s, pdev_ptr).state != PDEV_STOPPED) &&
       !(PdevAt(old_s, pdev_ptr).p2p_stream_valid == RMM_TRUE))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, pdev_ptr).state == GranuleAt(old_s, pdev_ptr).state)
}