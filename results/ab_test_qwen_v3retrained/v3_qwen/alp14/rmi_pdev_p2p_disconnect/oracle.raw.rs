pub open spec fn rmi_pdev_p2p_disconnect_spec(stream_ptr: Address, pdev_1_ptr: Address, pdev_2_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsGranuleAligned(old_s, stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, stream_ptr).state != P2P_STREAM ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, pdev_1_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, pdev_1_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, pdev_1_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, pdev_2_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, pdev_2_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, pdev_2_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((PdevAt(old_s, pdev_1_ptr).p2p_stream_valid != RMM_TRUE || PdevAt(old_s, pdev_1_ptr).p2p_stream != stream_ptr || PdevAt(old_s, pdev_2_ptr).p2p_stream_valid != RMM_TRUE || PdevAt(old_s, pdev_2_ptr).p2p_stream != stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> GranuleAt(new_s, stream_ptr).state == DELEGATED)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_1_ptr).p2p_stream_valid == RMM_FALSE)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_1_ptr).state == PDEV_COMMUNICATING)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_1_ptr).comm_state == DEV_COMM_PENDING)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_2_ptr).p2p_stream_valid == RMM_FALSE)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_2_ptr).state == PDEV_COMMUNICATING)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_2_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       AddrIsGranuleAligned(old_s, stream_ptr) &&
       PaIsDelegable(old_s, stream_ptr) &&
       !(GranuleAt(old_s, stream_ptr).state != P2P_STREAM) &&
       AddrIsGranuleAligned(old_s, pdev_1_ptr) &&
       PaIsDelegable(old_s, pdev_1_ptr) &&
       !(GranuleAt(old_s, pdev_1_ptr).state != PDEV) &&
       AddrIsGranuleAligned(old_s, pdev_2_ptr) &&
       PaIsDelegable(old_s, pdev_2_ptr) &&
       !(GranuleAt(old_s, pdev_2_ptr).state != PDEV) &&
       !((PdevAt(old_s, pdev_1_ptr).p2p_stream_valid != RMM_TRUE || PdevAt(old_s, pdev_1_ptr).p2p_stream != stream_ptr || PdevAt(old_s, pdev_2_ptr).p2p_stream_valid != RMM_TRUE || PdevAt(old_s, pdev_2_ptr).p2p_stream != stream_ptr)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, stream_ptr).state == GranuleAt(old_s, stream_ptr).state)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_1_ptr).p2p_stream_valid == PdevAt(old_s, pdev_1_ptr).p2p_stream_valid)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_1_ptr).state == PdevAt(old_s, pdev_1_ptr).state)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_1_ptr).comm_state == PdevAt(old_s, pdev_1_ptr).comm_state)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_2_ptr).p2p_stream_valid == PdevAt(old_s, pdev_2_ptr).p2p_stream_valid)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_2_ptr).state == PdevAt(old_s, pdev_2_ptr).state)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_2_ptr).comm_state == PdevAt(old_s, pdev_2_ptr).comm_state)
}