pub open spec fn rsi_vdev_dma_enable_spec(vdev_id: Bits64, flags: RsiVdevDmaFlags, non_ats_plane: UInt64, lock_seq: UInt64, meas_seq: UInt64, report_seq: UInt64, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (CurrentRealm(old_s).feat_da != FEATURE_TRUE ==> result == RSI_ERROR_STATE)
  && (VdevIdIsFree(old_s, CurrentRealm(old_s), vdev_id) ==> result == RSI_ERROR_INPUT)
  && (non_ats_plane > CurrentRealm(old_s).num_aux_planes ==> result == RSI_ERROR_INPUT)
  && (!VdevFreshnessEqual(lock_seq,meas_seq,report_seq,VdevFromVdevId(old_s, CurrentRealm(old_s),vdev_id).freshness) ==> result == RSI_ERROR_DEVICE)
  && (VdevFromVdevId(old_s, CurrentRealm(old_s),vdev_id).vdev_state != VDEV_STARTED ==> result == RSI_ERROR_DEVICE)
  && (result == RSI_INCOMPLETE ==> result == RSI_INCOMPLETE)
  && (result == RSI_SUCCESS && VdevFromVdevId(old_s, CurrentRealm(old_s),vdev_id).dma_state == VDEV_DMA_ENABLED && VdevFromVdevId(old_s, CurrentRealm(old_s),vdev_id).non_ats_plane == non_ats_plane ==> result == RSI_SUCCESS && VdevFromVdevId(new_s, CurrentRealm(new_s),vdev_id).dma_state == VDEV_DMA_ENABLED && VdevFromVdevId(new_s, CurrentRealm(new_s),vdev_id).non_ats_plane == non_ats_plane)
  && ((!(CurrentRealm(old_s).feat_da != FEATURE_TRUE) &&
       !(VdevIdIsFree(old_s, CurrentRealm(old_s), vdev_id)) &&
       !(non_ats_plane > CurrentRealm(old_s).num_aux_planes) &&
       VdevFreshnessEqual(lock_seq,meas_seq,report_seq,VdevFromVdevId(old_s, CurrentRealm(old_s),vdev_id).freshness) &&
       !(VdevFromVdevId(old_s, CurrentRealm(old_s),vdev_id).vdev_state != VDEV_STARTED))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> VdevFromVdevId(new_s, CurrentRealm(new_s),vdev_id).dma_state == VdevFromVdevId(old_s, CurrentRealm(old_s),vdev_id).dma_state)
  && (result != RSI_SUCCESS
    ==> VdevFromVdevId(new_s, CurrentRealm(new_s),vdev_id).non_ats_plane == VdevFromVdevId(old_s, CurrentRealm(old_s),vdev_id).non_ats_plane)
}