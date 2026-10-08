pub open spec fn rmi_vdev_p2p_bind_spec(stream_ptr: Address, rd: Address, rec_ptr: Address, pdev_1_ptr: Address, pdev_2_ptr: Address, vdev_1_ptr: Address, vdev_2_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (AddrIsGranuleAligned(old_s, rd) ==> result.is_Ok())
  && (GranuleAt(old_s, rd).state == RD ==> result.is_Ok())
  && (AddrIsGranuleAligned(old_s, rec_ptr) ==> result.is_Ok())
  && (GranuleAt(old_s, rec_ptr).state == REC ==> result.is_Ok())
  && (RecAt(old_s, rec_ptr).state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
  && (RecAt(old_s, rec_ptr).state != REC_RUNNING ==> result.is_Ok())
  && (RecAt(old_s, rec_ptr).owner != rd ==> ResultEqual(result, RMI_ERROR_REC))
  && (RecAt(old_s, rec_ptr).owner == rd ==> result.is_Ok())
  && (AddrIsGranuleAligned(old_s, stream_ptr) ==> result.is_Ok())
  && (GranuleAt(old_s, stream_ptr).state == P2P_STREAM ==> result.is_Ok())
  && (AddrIsGranuleAligned(old_s, pdev_1_ptr) ==> result.is_Ok())
  && (GranuleAt(old_s, pdev_1_ptr).state == PDEV ==> result.is_Ok())
  && (PdevAt(old_s, pdev_1_ptr).p2p_stream_valid == RMM_TRUE ==> result.is_Ok())
  && (PdevAt(old_s, pdev_1_ptr).p2p_stream == stream_ptr ==> result.is_Ok())
  && (AddrIsGranuleAligned(old_s, pdev_2_ptr) ==> result.is_Ok())
  && (GranuleAt(old_s, pdev_2_ptr).state == PDEV ==> result.is_Ok())
  && (PdevAt(old_s, pdev_2_ptr).p2p_stream_valid == RMM_TRUE ==> result.is_Ok())
  && (PdevAt(old_s, pdev_2_ptr).p2p_stream == stream_ptr ==> result.is_Ok())
  && (AddrIsGranuleAligned(old_s, vdev_1_ptr) ==> result.is_Ok())
  && (GranuleAt(old_s, vdev_1_ptr).state == VDEV ==> result.is_Ok())
  && (VdevAt(old_s, vdev_1_ptr).realm == RealmAt(old_s, rd) ==> result.is_Ok())
  && (VdevAt(old_s, vdev_1_ptr).pdev == pdev_1_ptr ==> result.is_Ok())
  && (VdevAt(old_s, vdev_1_ptr).comm_state == DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (VdevAt(old_s, vdev_1_ptr).comm_state != DEV_COMM_IDLE ==> result.is_Ok())
  && (VdevAt(old_s, vdev_1_ptr).p2p_bound == FEATURE_FALSE ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (VdevAt(old_s, vdev_1_ptr).p2p_bound != FEATURE_FALSE ==> result.is_Ok())
  && (AddrIsGranuleAligned(old_s, vdev_2_ptr) ==> result.is_Ok())
  && (GranuleAt(old_s, vdev_2_ptr).state == VDEV ==> result.is_Ok())
  && (VdevAt(old_s, vdev_2_ptr).realm == RealmAt(old_s, rd) ==> result.is_Ok())
  && (VdevAt(old_s, vdev_2_ptr).pdev == pdev_2_ptr ==> result.is_Ok())
  && (VdevAt(old_s, vdev_2_ptr).comm_state == DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (VdevAt(old_s, vdev_2_ptr).comm_state != DEV_COMM_IDLE ==> result.is_Ok())
  && (VdevAt(old_s, vdev_2_ptr).p2p_bound == FEATURE_FALSE ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (VdevAt(old_s, vdev_2_ptr).p2p_bound != FEATURE_FALSE ==> result.is_Ok())
  && (result.is_Ok() ==> VdevAt(new_s, vdev_1_ptr).op == VDEV_OP_P2P_BIND)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_1_ptr).comm_state == DEV_COMM_PENDING)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_1_ptr).p2p_bound == FEATURE_TRUE)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_1_ptr).p2p_stream == stream_ptr)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_1_ptr).p2p_peer == VdevAt(new_s, vdev_2_ptr).vdev_id)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_2_ptr).op == VDEV_OP_P2P_BIND)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_2_ptr).comm_state == DEV_COMM_PENDING)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_2_ptr).p2p_bound == FEATURE_TRUE)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_2_ptr).p2p_stream == stream_ptr)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_2_ptr).p2p_peer == VdevAt(new_s, vdev_1_ptr).vdev_id)
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       AddrIsGranuleAligned(old_s, rd) &&
       GranuleAt(old_s, rd).state == RD &&
       AddrIsGranuleAligned(old_s, rec_ptr) &&
       GranuleAt(old_s, rec_ptr).state == REC &&
       !(RecAt(old_s, rec_ptr).state == REC_RUNNING) &&
       RecAt(old_s, rec_ptr).state != REC_RUNNING &&
       RecAt(old_s, rec_ptr).owner == RecAt(old_s, rec_ptr).owner &&
       AddrIsGranuleAligned(old_s, stream_ptr) &&
       GranuleAt(old_s, stream_ptr).state == P2P_STREAM &&
       AddrIsGranuleAligned(old_s, pdev_1_ptr) &&
       GranuleAt(old_s, pdev_1_ptr).state == PDEV &&
       PdevAt(old_s, pdev_1_ptr).p2p_stream_valid == RMM_TRUE &&
       PdevAt(old_s, pdev_1_ptr).p2p_stream == stream_ptr &&
       AddrIsGranuleAligned(old_s, pdev_2_ptr) &&
       GranuleAt(old_s, pdev_2_ptr).state == PDEV &&
       PdevAt(old_s, pdev_2_ptr).p2p_stream_valid == RMM_TRUE &&
       PdevAt(old_s, pdev_2_ptr).p2p_stream == stream_ptr &&
       AddrIsGranuleAligned(old_s, vdev_1_ptr) &&
       GranuleAt(old_s, vdev_1_ptr).state == VDEV &&
       VdevAt(old_s, vdev_1_ptr).realm == RealmAt(old_s, rd) &&
       VdevAt(old_s, vdev_1_ptr).pdev == pdev_1_ptr &&
       VdevAt(old_s, vdev_1_ptr).comm_state == DEV_COMM_IDLE &&
       VdevAt(old_s, vdev_1_ptr).comm_state != DEV_COMM_IDLE &&
       VdevAt(old_s, vdev_1_ptr).p2p_bound == FEATURE_FALSE &&
       VdevAt(old_s, vdev_1_ptr).p2p_bound != FEATURE_FALSE &&
       AddrIsGranuleAligned(old_s, vdev_2_ptr) &&
       GranuleAt(old_s, vdev_2_ptr).state == VDEV &&
       VdevAt(old_s, vdev_2_ptr).realm == RealmAt(old_s, rd) &&
       VdevAt(old_s, vdev_2_ptr).pdev == pdev_2_ptr &&
       VdevAt(old_s, vdev_2_ptr).comm_state == DEV_COMM_IDLE &&
       VdevAt(old_s, vdev_2_ptr).comm_state != DEV_COMM_IDLE &&
       VdevAt(old_s, vdev_2_ptr).p2p_bound == FEATURE_FALSE &&
       VdevAt(old_s, vdev_2_ptr).p2p_bound != FEATURE_FALSE)
    ==> result.is_Ok())
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_1_ptr).op == VdevAt(old_s, vdev_1_ptr).op)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_1_ptr).comm_state == VdevAt(old_s, vdev_1_ptr).comm_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_1_ptr).p2p_bound == VdevAt(old_s, vdev_1_ptr).p2p_bound)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_1_ptr).p2p_stream == VdevAt(old_s, vdev_1_ptr).p2p_stream)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_1_ptr).p2p_peer == VdevAt(old_s, vdev_1_ptr).p2p_peer)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_2_ptr).op == VdevAt(old_s, vdev_2_ptr).op)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_2_ptr).comm_state == VdevAt(old_s, vdev_2_ptr).comm_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_2_ptr).p2p_bound == VdevAt(old_s, vdev_2_ptr).p2p_bound)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_2_ptr).p2p_stream == VdevAt(old_s, vdev_2_ptr).p2p_stream)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_2_ptr).p2p_peer == VdevAt(old_s, vdev_2_ptr).p2p_peer)
}