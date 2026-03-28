pub open spec fn rsi_rdev_p2p_bind_spec(vdev_id_1: Bits64, inst_id_1: UInt64, vdev_id_2: Bits64, inst_id_2: UInt64, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (CurrentRealm(old_s).feat_da != FEATURE_TRUE ==> result == RSI_ERROR_STATE)
  && (!VdevIdsAreValid(old_s, CurrentRealm(old_s), vdev_id_1, inst_id_1 as int) ==> result == RSI_ERROR_INPUT)
  && (!VdevIdsAreValid(old_s, CurrentRealm(old_s), vdev_id_2, inst_id_2 as int) ==> result == RSI_ERROR_INPUT)
  && ((PdevAt(old_s, VdevFromVdevId(old_s, CurrentRealm(old_s),vdev_id_1).pdev).p2p_added == RMM_FALSE || PdevAt(old_s, VdevFromVdevId(old_s, CurrentRealm(old_s),vdev_id_2).pdev).p2p_added == RMM_FALSE) ==> result == RSI_ERROR_INPUT)
  && (PdevAt(old_s, VdevFromVdevId(old_s, CurrentRealm(old_s),vdev_id_1).pdev).p2p_addr != PdevAt(old_s, VdevFromVdevId(old_s, CurrentRealm(old_s),vdev_id_2).pdev).p2p_addr ==> result == RSI_ERROR_INPUT)
  && ((!(CurrentRealm(old_s).feat_da != FEATURE_TRUE) &&
       VdevIdsAreValid(old_s, CurrentRealm(old_s), vdev_id_1, inst_id_1 as int) &&
       VdevIdsAreValid(old_s, CurrentRealm(old_s), vdev_id_2, inst_id_2 as int) &&
       !((PdevAt(old_s, VdevFromVdevId(old_s, CurrentRealm(old_s),vdev_id_1).pdev).p2p_added == RMM_FALSE || PdevAt(old_s, VdevFromVdevId(old_s, CurrentRealm(old_s),vdev_id_2).pdev).p2p_added == RMM_FALSE)) &&
       !(PdevAt(old_s, VdevFromVdevId(old_s, CurrentRealm(old_s),vdev_id_1).pdev).p2p_addr != PdevAt(old_s, VdevFromVdevId(old_s, CurrentRealm(old_s),vdev_id_2).pdev).p2p_addr))
    ==> result == RSI_SUCCESS)
}
