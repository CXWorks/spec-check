pub open spec fn rmi_pdev_p2p_connect_spec(stream_ptr: Address, pdev_1_ptr: Address, pdev_2_ptr: Address, ide_sid: UInt64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsGranuleAligned(old_s, stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegableDram(old_s, stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, stream_ptr).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, pdev_1_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, pdev_1_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, pdev_1_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (PdevAt(old_s, pdev_1_ptr).state != PDEV_READY ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (PdevAt(old_s, pdev_1_ptr).p2p_enabled != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (!AddrIsGranuleAligned(old_s, pdev_2_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, pdev_2_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, pdev_2_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (PdevAt(old_s, pdev_2_ptr).state != PDEV_READY ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (PdevAt(old_s, pdev_2_ptr).p2p_enabled != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (ide_sid > 255 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> GranuleAt(new_s, stream_ptr).state == P2P_STREAM)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_1_ptr).p2p_stream_valid == RMM_TRUE)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_1_ptr).p2p_stream == stream_ptr)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_1_ptr).state == PDEV_COMMUNICATING)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_1_ptr).comm_state == DEV_COMM_PENDING)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_2_ptr).p2p_stream_valid == RMM_TRUE)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_2_ptr).p2p_stream == stream_ptr)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_2_ptr).state == PDEV_COMMUNICATING)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_2_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       AddrIsGranuleAligned(old_s, stream_ptr) &&
       PaIsDelegableDram(old_s, stream_ptr) &&
       !(GranuleAt(old_s, stream_ptr).state != DELEGATED) &&
       AddrIsGranuleAligned(old_s, pdev_1_ptr) &&
       PaIsDelegable(old_s, pdev_1_ptr) &&
       !(GranuleAt(old_s, pdev_1_ptr).state != PDEV) &&
       !(PdevAt(old_s, pdev_1_ptr).state != PDEV_READY) &&
       !(PdevAt(old_s, pdev_1_ptr).p2p_enabled != FEATURE_TRUE) &&
       AddrIsGranuleAligned(old_s, pdev_2_ptr) &&
       PaIsDelegable(old_s, pdev_2_ptr) &&
       !(GranuleAt(old_s, pdev_2_ptr).state != PDEV) &&
       !(PdevAt(old_s, pdev_2_ptr).state != PDEV_READY) &&
       !(PdevAt(old_s, pdev_2_ptr).p2p_enabled != FEATURE_TRUE) &&
       !(ide_sid > 255))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, stream_ptr).state == GranuleAt(old_s, stream_ptr).state)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_1_ptr).p2p_stream_valid == PdevAt(old_s, pdev_1_ptr).p2p_stream_valid)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_1_ptr).p2p_stream == PdevAt(old_s, pdev_1_ptr).p2p_stream)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_1_ptr).state == PdevAt(old_s, pdev_1_ptr).state)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_1_ptr).comm_state == PdevAt(old_s, pdev_1_ptr).comm_state)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_2_ptr).p2p_stream_valid == PdevAt(old_s, pdev_2_ptr).p2p_stream_valid)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_2_ptr).p2p_stream == PdevAt(old_s, pdev_2_ptr).p2p_stream)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_2_ptr).state == PdevAt(old_s, pdev_2_ptr).state)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_2_ptr).comm_state == PdevAt(old_s, pdev_2_ptr).comm_state)
}
