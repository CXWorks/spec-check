pub open spec fn rmi_vdev_unlock_spec(rd: Address, pdev_ptr: Address, vdev_ptr: Address, result: Result<(), RmiStatusCode>, addr: Address, old_s: S, new_s: S) -> bool {
  (Rmm().static.feat_da != FEATURE_TRUE ==> result.status == RMI_ERROR_NOT_SUPPORTED)
  && (!AddrIsRmiGranuleAligned(old_s, rd) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsTracked(old_s, rd) ==> result.status == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, rd).state != GRAN_RD ==> result.status == RMI_ERROR_INPUT)
  && (!AddrIsRmiGranuleAligned(old_s, pdev_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsTracked(old_s, pdev_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, pdev_ptr).state != GRAN_PDEV ==> result.status == RMI_ERROR_INPUT)
  && (!AddrIsRmiGranuleAligned(old_s, vdev_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsTracked(old_s, vdev_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, vdev_ptr).state != GRAN_VDEV ==> result.status == RMI_ERROR_INPUT)
  && (VdevAt(old_s, vdev_ptr).realm != rd ==> result.status == RMI_ERROR_INPUT)
  && (VdevAt(old_s, vdev_ptr).pdev != pdev_ptr ==> result.status == RMI_ERROR_DEVICE)
  && ((VdevAt(old_s, vdev_ptr).vdev_state != VDEV_LOCKED && VdevAt(old_s, vdev_ptr).vdev_state != VDEV_STARTED && VdevAt(old_s, vdev_ptr).vdev_state != VDEV_ERROR) ==> result.status == RMI_ERROR_DEVICE)
  && (VdevAt(old_s, vdev_ptr).comm_state != DEV_COMM_IDLE ==> result.status == RMI_ERROR_DEVICE)
  && (VdevReqStreamRefresh(old_s, VdevAt(old_s, vdev_ptr)) ==> VdevAt(new_s, vdev_ptr).stream_refresh_cnt == VdevStreamRefreshCountSnapshot(new_s, VdevAt(new_s, vdev_ptr)))
  && (VdevReqStreamRefresh(old_s, VdevAt(old_s, vdev_ptr)) ==> VdevAt(new_s, vdev_ptr).vdev_state == VDEV_KEY_REFRESH)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_PENDING)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).dma_state == VDEV_DMA_DISABLED)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).op == VDEV_OP_UNLOCK)
  && ((!(Rmm().static.feat_da != FEATURE_TRUE) &&
       AddrIsRmiGranuleAligned(old_s, rd) &&
       PaIsTracked(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != GRAN_RD) &&
       AddrIsRmiGranuleAligned(old_s, pdev_ptr) &&
       PaIsTracked(old_s, pdev_ptr) &&
       !(GranuleAt(old_s, pdev_ptr).state != GRAN_PDEV) &&
       AddrIsRmiGranuleAligned(old_s, vdev_ptr) &&
       PaIsTracked(old_s, vdev_ptr) &&
       !(GranuleAt(old_s, vdev_ptr).state != GRAN_VDEV) &&
       !(VdevAt(old_s, vdev_ptr).realm != rd) &&
       !(VdevAt(old_s, vdev_ptr).pdev != pdev_ptr) &&
       !((VdevAt(old_s, vdev_ptr).vdev_state != VDEV_LOCKED && VdevAt(old_s, vdev_ptr).vdev_state != VDEV_STARTED && VdevAt(old_s, vdev_ptr).vdev_state != VDEV_ERROR)) &&
       !(VdevAt(old_s, vdev_ptr).comm_state != DEV_COMM_IDLE) &&
       !(VdevReqStreamRefresh(old_s, VdevAt(old_s, vdev_ptr))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).dma_state == VdevAt(old_s, vdev_ptr).dma_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).op == VdevAt(old_s, vdev_ptr).op)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).comm_state == VdevAt(old_s, vdev_ptr).comm_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).vdev_state == VdevAt(old_s, vdev_ptr).vdev_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).stream_refresh_cnt == VdevAt(old_s, vdev_ptr).stream_refresh_cnt)
}