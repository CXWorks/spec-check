pub open spec fn rsi_vdev_dma_disable_spec(vdev_id: Bits64, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (CurrentRealm(old_s).feat_da != FEATURE_TRUE ==> result == RSI_ERROR_STATE)
  && (VdevIdIsFree(old_s, CurrentRealm(old_s), vdev_id) ==> result == RSI_ERROR_INPUT)
  && (result == RSI_INCOMPLETE ==> result == RSI_INCOMPLETE)
  && (result == RSI_SUCCESS && VdevFromVdevId(new_s, CurrentRealm(new_s), vdev_id).dma_state == VDEV_DMA_DISABLED ==> result == RSI_SUCCESS && VdevFromVdevId(new_s, CurrentRealm(new_s), vdev_id).dma_state == VDEV_DMA_DISABLED)
  && ((!(CurrentRealm(old_s).feat_da != FEATURE_TRUE) &&
       !(VdevIdIsFree(old_s, CurrentRealm(old_s), vdev_id)))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> VdevFromVdevId(new_s, CurrentRealm(new_s), vdev_id).dma_state == VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).dma_state)
}