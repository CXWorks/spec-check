pub open spec fn rmi_vdev_unlock_spec(rd: Address, vdev_ptr: Address, result: Result<(), RmiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> result == RMI_ERROR_NOT_SUPPORTED)
  && (!AddrIsGranuleAligned(old_s, rd) ==> result == RMI_ERROR_INPUT)
  && (!PaIsDelegable(old_s, rd) ==> result == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, rd).state != RD ==> result == RMI_ERROR_INPUT)
  && (!AddrIsGranuleAligned(old_s, vdev_ptr) ==> result == RMI_ERROR_INPUT)
  && (!PaIsDelegable(old_s, vdev_ptr) ==> result == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, vdev_ptr).state != VDEV ==> result == RMI_ERROR_INPUT)
  && (VdevAt(old_s, vdev_ptr).realm != rd ==> result == RMI_ERROR_INPUT)
  && ((VdevAt(old_s, vdev_ptr).vdev_state != VDEV_LOCKED && VdevAt(old_s, vdev_ptr).vdev_state != VDEV_STARTED && VdevAt(old_s, vdev_ptr).vdev_state != VDEV_ERROR) ==> result == RMI_ERROR_DEVICE)
  && (VdevAt(old_s, vdev_ptr).comm_state != DEV_COMM_IDLE ==> result == RMI_ERROR_DEVICE)
  && (VdevAt(old_s, vdev_ptr).num_map != 0 ==> result == RMI_ERROR_DEVICE)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).dma_state == VDEV_DMA_DISABLED)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).op == VDEV_OP_UNLOCK)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       AddrIsGranuleAligned(old_s, vdev_ptr) &&
       PaIsDelegable(old_s, vdev_ptr) &&
       !(GranuleAt(old_s, vdev_ptr).state != VDEV) &&
       !(VdevAt(old_s, vdev_ptr).realm != rd) &&
       !((VdevAt(old_s, vdev_ptr).vdev_state != VDEV_LOCKED && VdevAt(old_s, vdev_ptr).vdev_state != VDEV_STARTED && VdevAt(old_s, vdev_ptr).vdev_state != VDEV_ERROR)) &&
       !(VdevAt(old_s, vdev_ptr).comm_state != DEV_COMM_IDLE) &&
       !(VdevAt(old_s, vdev_ptr).num_map != 0))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).dma_state == VdevAt(old_s, vdev_ptr).dma_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).op == VdevAt(old_s, vdev_ptr).op)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).comm_state == VdevAt(old_s, vdev_ptr).comm_state)
}