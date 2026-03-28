pub open spec fn rmi_p2p_bind_spec(rd: Address, stream_ptr: Address, pdev1_ptr: Address, pdev2_ptr: Address, vdev1_ptr: Address, vdev2_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, stream_ptr).state != P2P_STREAM ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, pdev1_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, pdev1_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, pdev1_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((PdevAt(old_s, pdev1_ptr).p2p_added != RMM_TRUE || PdevAt(old_s, pdev1_ptr).p2p_addr != stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, pdev2_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, pdev2_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, pdev2_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((PdevAt(old_s, pdev2_ptr).p2p_added != RMM_TRUE || PdevAt(old_s, pdev2_ptr).p2p_addr != stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, vdev1_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, vdev1_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, vdev1_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (VdevAt(old_s, vdev1_ptr).realm != rd ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (VdevAt(old_s, vdev1_ptr).pdev != pdev1_ptr ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (VdevAt(old_s, vdev1_ptr).comm_state != DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (!AddrIsGranuleAligned(old_s, vdev2_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, vdev2_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, vdev2_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (VdevAt(old_s, vdev2_ptr).realm != rd ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (VdevAt(old_s, vdev2_ptr).pdev != pdev2_ptr ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (VdevAt(old_s, vdev2_ptr).comm_state != DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (result.is_Ok() ==> VdevAt(new_s, vdev1_ptr).op == VDEV_OP_P2P_BIND)
  && (result.is_Ok() ==> VdevAt(new_s, vdev1_ptr).comm_state == DEV_COMM_PENDING)
  && (result.is_Ok() ==> VdevAt(new_s, vdev2_ptr).op == VDEV_OP_P2P_BIND)
  && (result.is_Ok() ==> VdevAt(new_s, vdev2_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       AddrIsGranuleAligned(old_s, stream_ptr) &&
       PaIsDelegable(old_s, stream_ptr) &&
       !(GranuleAt(old_s, stream_ptr).state != P2P_STREAM) &&
       AddrIsGranuleAligned(old_s, pdev1_ptr) &&
       PaIsDelegable(old_s, pdev1_ptr) &&
       !(GranuleAt(old_s, pdev1_ptr).state != PDEV) &&
       !((PdevAt(old_s, pdev1_ptr).p2p_added != RMM_TRUE || PdevAt(old_s, pdev1_ptr).p2p_addr != stream_ptr)) &&
       AddrIsGranuleAligned(old_s, pdev2_ptr) &&
       PaIsDelegable(old_s, pdev2_ptr) &&
       !(GranuleAt(old_s, pdev2_ptr).state != PDEV) &&
       !((PdevAt(old_s, pdev2_ptr).p2p_added != RMM_TRUE || PdevAt(old_s, pdev2_ptr).p2p_addr != stream_ptr)) &&
       AddrIsGranuleAligned(old_s, vdev1_ptr) &&
       PaIsDelegable(old_s, vdev1_ptr) &&
       !(GranuleAt(old_s, vdev1_ptr).state != VDEV) &&
       !(VdevAt(old_s, vdev1_ptr).realm != rd) &&
       !(VdevAt(old_s, vdev1_ptr).pdev != pdev1_ptr) &&
       !(VdevAt(old_s, vdev1_ptr).comm_state != DEV_COMM_IDLE) &&
       AddrIsGranuleAligned(old_s, vdev2_ptr) &&
       PaIsDelegable(old_s, vdev2_ptr) &&
       !(GranuleAt(old_s, vdev2_ptr).state != VDEV) &&
       !(VdevAt(old_s, vdev2_ptr).realm != rd) &&
       !(VdevAt(old_s, vdev2_ptr).pdev != pdev2_ptr) &&
       !(VdevAt(old_s, vdev2_ptr).comm_state != DEV_COMM_IDLE))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> VdevAt(new_s, vdev1_ptr).op == VdevAt(old_s, vdev1_ptr).op)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev1_ptr).comm_state == VdevAt(old_s, vdev1_ptr).comm_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev2_ptr).op == VdevAt(old_s, vdev2_ptr).op)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev2_ptr).comm_state == VdevAt(old_s, vdev2_ptr).comm_state)
}
