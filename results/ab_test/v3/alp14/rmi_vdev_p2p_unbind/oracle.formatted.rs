pub open spec fn rmi_vdev_p2p_unbind_spec(stream_ptr: Address, rd: Address, rec_ptr: Address, pdev_1_ptr: Address, pdev_2_ptr: Address, vdev_1_ptr: Address, vdev_2_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RecAt(old_s, rec_ptr).state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
  && (RecAt(old_s, rec_ptr).owner != rd ==> ResultEqual(result, RMI_ERROR_REC))
  && (!AddrIsGranuleAligned(old_s, stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, stream_ptr).state != P2P_STREAM ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, pdev_1_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, pdev_1_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, pdev_1_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((PdevAt(old_s, pdev_1_ptr).p2p_stream_valid != RMM_TRUE || PdevAt(old_s, pdev_1_ptr).p2p_stream != stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, pdev_2_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, pdev_2_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, pdev_2_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((PdevAt(old_s, pdev_2_ptr).p2p_stream_valid != RMM_TRUE || PdevAt(old_s, pdev_2_ptr).p2p_stream != stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, vdev_1_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, vdev_1_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, vdev_1_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (VdevAt(old_s, vdev_1_ptr).realm != rd ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (VdevAt(old_s, vdev_1_ptr).pdev != pdev_1_ptr ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (VdevAt(old_s, vdev_1_ptr).comm_state != DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (VdevAt(old_s, vdev_1_ptr).p2p_bound != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (VdevAt(old_s, vdev_1_ptr).p2p_stream != stream_ptr ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (VdevAt(old_s, vdev_1_ptr).p2p_peer != VdevAt(old_s, vdev_2_ptr).vdev_id ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (!AddrIsGranuleAligned(old_s, vdev_2_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, vdev_2_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, vdev_2_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (VdevAt(old_s, vdev_2_ptr).realm != rd ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (VdevAt(old_s, vdev_2_ptr).pdev != pdev_2_ptr ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (VdevAt(old_s, vdev_2_ptr).comm_state != DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (VdevAt(old_s, vdev_2_ptr).p2p_bound != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (VdevAt(old_s, vdev_2_ptr).p2p_stream != stream_ptr ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (VdevAt(old_s, vdev_2_ptr).p2p_peer != VdevAt(old_s, vdev_1_ptr).vdev_id ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (result.is_Ok() ==> VdevAt(new_s, vdev_1_ptr).op == VDEV_OP_P2P_UNBIND)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_1_ptr).comm_state == DEV_COMM_PENDING)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_1_ptr).p2p_bound == FEATURE_FALSE)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_2_ptr).op == VDEV_OP_P2P_UNBIND)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_2_ptr).comm_state == DEV_COMM_PENDING)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_2_ptr).p2p_bound == FEATURE_FALSE)
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       AddrIsGranuleAligned(old_s, rec_ptr) &&
       PaIsDelegable(old_s, rec_ptr) &&
       !(GranuleAt(old_s, rec_ptr).state != REC) &&
       !(RecAt(old_s, rec_ptr).state == REC_RUNNING) &&
       !(RecAt(old_s, rec_ptr).owner != rd) &&
       AddrIsGranuleAligned(old_s, stream_ptr) &&
       PaIsDelegable(old_s, stream_ptr) &&
       !(GranuleAt(old_s, stream_ptr).state != P2P_STREAM) &&
       AddrIsGranuleAligned(old_s, pdev_1_ptr) &&
       PaIsDelegable(old_s, pdev_1_ptr) &&
       !(GranuleAt(old_s, pdev_1_ptr).state != PDEV) &&
       !((PdevAt(old_s, pdev_1_ptr).p2p_stream_valid != RMM_TRUE || PdevAt(old_s, pdev_1_ptr).p2p_stream != stream_ptr)) &&
       AddrIsGranuleAligned(old_s, pdev_2_ptr) &&
       PaIsDelegable(old_s, pdev_2_ptr) &&
       !(GranuleAt(old_s, pdev_2_ptr).state != PDEV) &&
       !((PdevAt(old_s, pdev_2_ptr).p2p_stream_valid != RMM_TRUE || PdevAt(old_s, pdev_2_ptr).p2p_stream != stream_ptr)) &&
       AddrIsGranuleAligned(old_s, vdev_1_ptr) &&
       PaIsDelegable(old_s, vdev_1_ptr) &&
       !(GranuleAt(old_s, vdev_1_ptr).state != VDEV) &&
       !(VdevAt(old_s, vdev_1_ptr).realm != rd) &&
       !(VdevAt(old_s, vdev_1_ptr).pdev != pdev_1_ptr) &&
       !(VdevAt(old_s, vdev_1_ptr).comm_state != DEV_COMM_IDLE) &&
       !(VdevAt(old_s, vdev_1_ptr).p2p_bound != FEATURE_TRUE) &&
       !(VdevAt(old_s, vdev_1_ptr).p2p_stream != stream_ptr) &&
       !(VdevAt(old_s, vdev_1_ptr).p2p_peer != VdevAt(old_s, vdev_2_ptr).vdev_id) &&
       AddrIsGranuleAligned(old_s, vdev_2_ptr) &&
       PaIsDelegable(old_s, vdev_2_ptr) &&
       !(GranuleAt(old_s, vdev_2_ptr).state != VDEV) &&
       !(VdevAt(old_s, vdev_2_ptr).realm != rd) &&
       !(VdevAt(old_s, vdev_2_ptr).pdev != pdev_2_ptr) &&
       !(VdevAt(old_s, vdev_2_ptr).comm_state != DEV_COMM_IDLE) &&
       !(VdevAt(old_s, vdev_2_ptr).p2p_bound != FEATURE_TRUE) &&
       !(VdevAt(old_s, vdev_2_ptr).p2p_stream != stream_ptr) &&
       !(VdevAt(old_s, vdev_2_ptr).p2p_peer != VdevAt(old_s, vdev_1_ptr).vdev_id))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_1_ptr).op == VdevAt(old_s, vdev_1_ptr).op)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_1_ptr).comm_state == VdevAt(old_s, vdev_1_ptr).comm_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_1_ptr).p2p_bound == VdevAt(old_s, vdev_1_ptr).p2p_bound)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_2_ptr).op == VdevAt(old_s, vdev_2_ptr).op)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_2_ptr).comm_state == VdevAt(old_s, vdev_2_ptr).comm_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_2_ptr).p2p_bound == VdevAt(old_s, vdev_2_ptr).p2p_bound)
}