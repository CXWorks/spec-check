pub open spec fn rmi_p2p_stream_remove_spec(stream_ptr: Address, pdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsGranuleAligned(old_s, stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, stream_ptr).state != P2P_STREAM ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (PdevAt(old_s, pdev_ptr).state != PDEV_READY ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (PdevAt(old_s, pdev_ptr).p2p_added == RMM_FALSE ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (PdevAt(old_s, pdev_ptr).p2p_addr != stream_ptr ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (result.is_Ok() ==> P2PStreamAt(new_s, stream_ptr).num_pdevs == P2PStreamAt(new_s, stream_ptr).num_pdevs - 1)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).p2p_added == RMM_FALSE)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).state == PDEV_COMMUNICATING)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       AddrIsGranuleAligned(old_s, stream_ptr) &&
       PaIsDelegable(old_s, stream_ptr) &&
       !(GranuleAt(old_s, stream_ptr).state != P2P_STREAM) &&
       AddrIsGranuleAligned(old_s, pdev_ptr) &&
       PaIsDelegable(old_s, pdev_ptr) &&
       !(GranuleAt(old_s, pdev_ptr).state != PDEV) &&
       !(PdevAt(old_s, pdev_ptr).state != PDEV_READY) &&
       !(PdevAt(old_s, pdev_ptr).p2p_added == RMM_FALSE) &&
       !(PdevAt(old_s, pdev_ptr).p2p_addr != stream_ptr))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> P2PStreamAt(new_s, stream_ptr).num_pdevs == P2PStreamAt(old_s, stream_ptr).num_pdevs)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).p2p_added == PdevAt(old_s, pdev_ptr).p2p_added)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).state == PdevAt(old_s, pdev_ptr).state)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).comm_state == PdevAt(old_s, pdev_ptr).comm_state)
}
