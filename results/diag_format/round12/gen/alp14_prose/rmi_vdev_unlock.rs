pub open spec fn rmi_vdev_unlock_spec(rd: Address, vdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (result.is_Err() ==> VdevAt(new_s, vdev_ptr).dma_state == VDEV_DMA_DISABLED)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).dma_state == VDEV_DMA_DISABLED)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).op == VDEV_OP_UNLOCK)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(VdevAt(old_s, vdev_ptr).dma_state == VDEV_DMA_DISABLED))
    ==> VdevAt(new_s, vdev_ptr).dma_state == VdevAt(old_s, vdev_ptr).dma_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).op == VdevAt(old_s, vdev_ptr).op)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).comm_state == VdevAt(old_s, vdev_ptr).comm_state)
  && (result.is_Ok()
    ==> VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_PENDING)
}