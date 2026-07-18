pub open spec fn rsi_vdev_get_info_spec(vdev_id: Bits64, addr: Address, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (CurrentRealm(old_s).feat_da != FEATURE_TRUE ==> result == RSI_ERROR_STATE)
  && (VdevIdIsFree(old_s, CurrentRealm(old_s), vdev_id) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsAligned(old_s, addr, 512) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsProtected(old_s, addr, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
  && (RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY).rtte.ripas == EMPTY ==> result == RSI_ERROR_INPUT)
  && (result != RSI_ERROR_INPUT && result != RSI_ERROR_STATE ==> Equal(RsiVdevInfoAt(new_s, addr).hash_algo, PdevAt(new_s, VdevFromVdevId(new_s, CurrentRealm(new_s),vdev_id).pdev).hash_algo))
  && (result != RSI_ERROR_INPUT && result != RSI_ERROR_STATE ==> Equal(RsiVdevInfoAt(new_s, addr).flags.p2p_enabled, PdevAt(new_s, VdevFromVdevId(new_s, CurrentRealm(new_s),vdev_id).pdev).p2p_enabled))
  && (result != RSI_ERROR_INPUT && result != RSI_ERROR_STATE ==> Equal(RsiVdevInfoAt(new_s, addr).flags.p2p_bound, VdevFromVdevId(new_s, CurrentRealm(new_s),vdev_id).p2p_bound))
  && (result != RSI_ERROR_INPUT && result != RSI_ERROR_STATE ==> RsiVdevInfoAt(new_s, addr).p2p_peer == VdevFromVdevId(new_s, CurrentRealm(new_s),vdev_id).p2p_peer)
  && (result != RSI_ERROR_INPUT && result != RSI_ERROR_STATE ==> VdevAttestInfoEqual(RsiVdevInfoAt(new_s, addr).lock_nonce,RsiVdevInfoAt(new_s, addr).meas_nonce,RsiVdevInfoAt(new_s, addr).report_nonce,VdevFromVdevId(new_s, CurrentRealm(new_s),vdev_id).attest_info))
  && (result != RSI_ERROR_INPUT && result != RSI_ERROR_STATE ==> RsiVdevInfoAt(new_s, addr).vca_digest == PdevAt(new_s, VdevFromVdevId(new_s, CurrentRealm(new_s),vdev_id).pdev).vca_digest)
  && (result != RSI_ERROR_INPUT && result != RSI_ERROR_STATE ==> RsiVdevInfoAt(new_s, addr).meas_digest == VdevFromVdevId(new_s, CurrentRealm(new_s),vdev_id).meas_digest)
  && (result != RSI_ERROR_INPUT && result != RSI_ERROR_STATE ==> RsiVdevInfoAt(new_s, addr).report_digest == VdevFromVdevId(new_s, CurrentRealm(new_s),vdev_id).report_digest)
  && (result != RSI_ERROR_INPUT && result != RSI_ERROR_STATE ==> Equal(RsiVdevInfoAt(new_s, addr).state, VdevFromVdevId(new_s, CurrentRealm(new_s),vdev_id).vdev_state))
  && ((!(CurrentRealm(old_s).feat_da != FEATURE_TRUE) &&
       !(VdevIdIsFree(old_s, CurrentRealm(old_s), vdev_id)) &&
       AddrIsAligned(old_s, addr, 512) &&
       AddrIsProtected(old_s, addr, CurrentRealm(old_s)) &&
       !(RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY).rtte.ripas == EMPTY))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).state == RSI_VDEV_INFO_STATE_ERROR)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).meas_digest == RSI_VDEV_MEAS_ERROR)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).report_digest == RSI_VDEV_REPORT_ERROR)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).vca_digest == RSI_VDEV_VCA_ERROR)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).p2p_peer == 0)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).lock_nonce == 0)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).meas_nonce == 0)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).report_nonce == 0)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).flags.p2p_enabled == 0)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).flags.p2p_bound == 0)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).hash_algo == 0)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).state == 0)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).meas_digest == 0)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).report_digest == 0)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).vca_digest == 0)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).p2p_peer == 0)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).lock_nonce == 0)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).meas_nonce == 0)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).report_nonce == 0)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).flags.p2p_enabled == 0)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).flags.p2p_bound == 0)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).hash_algo == 0)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).state == 0)
}