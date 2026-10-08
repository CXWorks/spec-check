pub open spec fn rsi_vdev_get_info_spec(vdev_id: Bits64, addr: Address, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (!CurrentRealm(old_s).feat_p2p ==> ResultEqual(result, RSI_ERROR_STATE))
  && (!VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).pdev == PdevAt(old_s, VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).pdev).pdev_id ==> ResultEqual(result, RSI_ERROR_INPUT))
  && (!AddrIsAligned(old_s, addr, 9) ==> ResultEqual(result, RSI_ERROR_INPUT))
  && (!AddrIsProtected(old_s, addr, CurrentRealm(old_s)) ==> ResultEqual(result, RSI_ERROR_INPUT))
  && (PdevAt(old_s, VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).pdev).ripas == RmmRipas::EMPTY ==> ResultEqual(result, RSI_ERROR_INPUT))
  && (result == RSI_SUCCESS ==> RsiVdevInfoAt(new_s, addr).state == VdevFromVdevId(new_s, CurrentRealm(new_s), vdev_id).vdev_state)
  && (result == RSI_SUCCESS ==> RsiVdevInfoAt(new_s, addr).p2p_enabled == PdevAt(new_s, VdevFromVdevId(new_s, CurrentRealm(new_s), vdev_id).pdev).p2p_enabled)
  && (result == RSI_SUCCESS ==> RsiVdevInfoAt(new_s, addr).p2p_bound == VdevFromVdevId(new_s, CurrentRealm(new_s), vdev_id).p2p_bound)
  && (result == RSI_SUCCESS ==> RsiVdevInfoAt(new_s, addr).p2p_peer == VdevFromVdevId(new_s, CurrentRealm(new_s), vdev_id).p2p_peer)
  && (result == RSI_SUCCESS ==> RsiVdevInfoAt(new_s, addr).lock_nonce == VdevFromVdevId(new_s, CurrentRealm(new_s), vdev_id).attest_info.lock_nonce)
  && (result == RSI_SUCCESS ==> RsiVdevInfoAt(new_s, addr).meas_nonce == VdevFromVdevId(new_s, CurrentRealm(new_s), vdev_id).attest_info.meas_nonce)
  && (result == RSI_SUCCESS ==> RsiVdevInfoAt(new_s, addr).report_nonce == VdevFromVdevId(new_s, CurrentRealm(new_s), vdev_id).attest_info.report_nonce)
  && (result == RSI_SUCCESS ==> RsiVdevInfoAt(new_s, addr).vca_digest == PdevAt(new_s, VdevFromVdevId(new_s, CurrentRealm(new_s), vdev_id).pdev).vca_digest)
  && (result == RSI_SUCCESS ==> RsiVdevInfoAt(new_s, addr).meas_digest == VdevFromVdevId(new_s, CurrentRealm(new_s), vdev_id).meas_digest)
  && (result == RSI_SUCCESS ==> RsiVdevInfoAt(new_s, addr).report_digest == VdevFromVdevId(new_s, CurrentRealm(new_s), vdev_id).report_digest)
  && ((!(CurrentRealm(old_s).feat_p2p) &&
       VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).pdev == PdevAt(old_s, VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).pdev).pdev_id &&
       AddrIsAligned(old_s, addr, 9) &&
       AddrIsProtected(old_s, addr, CurrentRealm(old_s)) &&
       !(PdevAt(old_s, VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).pdev).ripas == RmmRipas::EMPTY))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).state == RsiVdevInfoAt(old_s, addr).state)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).p2p_enabled == RsiVdevInfoAt(old_s, addr).p2p_enabled)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).p2p_bound == RsiVdevInfoAt(old_s, addr).p2p_bound)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).p2p_peer == RsiVdevInfoAt(old_s, addr).p2p_peer)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).lock_nonce == RsiVdevInfoAt(old_s, addr).lock_nonce)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).meas_nonce == RsiVdevInfoAt(old_s, addr).meas_nonce)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).report_nonce == RsiVdevInfoAt(old_s, addr).report_nonce)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).vca_digest == RsiVdevInfoAt(old_s, addr).vca_digest)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).meas_digest == RsiVdevInfoAt(old_s, addr).meas_digest)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).report_digest == RsiVdevInfoAt(old_s, addr).report_digest)
}