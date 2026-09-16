pub open spec fn rsi_vdev_get_info_spec(vdev_id: Bits64, addr: Address, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (CurrentRealm(old_s).feat_da != FEATURE_TRUE ==> result == RSI_ERROR_STATE)
  && (VdevIdIsFree(old_s, CurrentRealm(old_s), vdev_id) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsAligned(old_s, addr, 512) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsProtected(old_s, addr, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
  && (RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RIPAS_EMPTY ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> RsiVdevInfoAt(new_s, addr).id_index == PdevAt(new_s, VdevFromVdevId(new_s, CurrentRealm(new_s), CurrentRealm(new_s),vdev_id).pdev).id_index)
  && (result == RSI_SUCCESS ==> Equal(RsiVdevInfoAt(new_s, addr).hash_algo, PdevAt(new_s, VdevFromVdevId(new_s, CurrentRealm(new_s), CurrentRealm(new_s),vdev_id).pdev).hash_algo))
  && (result == RSI_SUCCESS ==> RsiVdevInfoAt(new_s, addr).tdisp_version == RsiVdevInfoAt(new_s, addr).tdisp_version)
  && (result == RSI_SUCCESS && RsiVdevInfoAt(old_s, addr).flags.protocol_data_set == RSI_TRUE ==> RsiVdevInfoAt(new_s, addr).protocol_data_digest == PdevAt(new_s, VdevFromVdevId(new_s, CurrentRealm(new_s), CurrentRealm(new_s),vdev_id).pdev).protocol_data_digest)
  && (result == RSI_SUCCESS ==> RsiVdevInfoAt(new_s, addr).meas_digest == VdevFromVdevId(new_s, CurrentRealm(new_s), CurrentRealm(new_s),vdev_id).meas_digest)
  && (result == RSI_SUCCESS ==> RsiVdevInfoAt(new_s, addr).report_digest == VdevFromVdevId(new_s, CurrentRealm(new_s), CurrentRealm(new_s),vdev_id).report_digest)
  && (result == RSI_SUCCESS ==> RsiVdevInfoAt(new_s, addr).state == VdevStateToRsi(VdevFromVdevId(new_s, CurrentRealm(new_s), CurrentRealm(new_s),vdev_id).vdev_state))
  && (result == RSI_SUCCESS ==> RsiVdevInfoAt(new_s, addr).flags.vsmmu == FeatureToRsi(VdevFromVdevId(new_s, CurrentRealm(new_s), CurrentRealm(new_s),vdev_id).vsmmu))
  && (result == RSI_SUCCESS && VdevFromVdevId(old_s, CurrentRealm(old_s), CurrentRealm(old_s),vdev_id).vsmmu == FEATURE_TRUE ==> RsiVdevInfoAt(new_s, addr).vsmmu_addr == VdevFromVdevId(new_s, CurrentRealm(new_s), CurrentRealm(new_s),vdev_id).vsmmu_reg_base)
  && (result == RSI_SUCCESS && VdevFromVdevId(old_s, CurrentRealm(old_s), CurrentRealm(old_s),vdev_id).vsmmu == FEATURE_TRUE ==> RsiVdevInfoAt(new_s, addr).vsmmu_vsid == VdevFromVdevId(new_s, CurrentRealm(new_s), CurrentRealm(new_s),vdev_id).vsid)
  && ((!(CurrentRealm(old_s).feat_da != FEATURE_TRUE) &&
       !(VdevIdIsFree(old_s, CurrentRealm(old_s), vdev_id)) &&
       AddrIsAligned(old_s, addr, 512) &&
       AddrIsProtected(old_s, addr, CurrentRealm(old_s)) &&
       !(RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RIPAS_EMPTY))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).id_index == RsiVdevInfoAt(old_s, addr).id_index)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).hash_algo == RsiVdevInfoAt(old_s, addr).hash_algo)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).tdisp_version == RsiVdevInfoAt(old_s, addr).tdisp_version)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).protocol_data_digest == RsiVdevInfoAt(old_s, addr).protocol_data_digest)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).meas_digest == RsiVdevInfoAt(old_s, addr).meas_digest)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).report_digest == RsiVdevInfoAt(old_s, addr).report_digest)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).state == RsiVdevInfoAt(old_s, addr).state)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).flags.vsmmu == RsiVdevInfoAt(old_s, addr).flags.vsmmu)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).vsmmu_addr == RsiVdevInfoAt(old_s, addr).vsmmu_addr)
  && (result != RSI_SUCCESS
    ==> RsiVdevInfoAt(new_s, addr).vsmmu_vsid == RsiVdevInfoAt(old_s, addr).vsmmu_vsid)
  && (RsiVdevInfoAt(new_s, addr).flags.protocol_data_set == RSI_FALSE
    ==> RsiVdevInfoAt(new_s, addr).protocol_data_digest == RsiVdevInfoAt(old_s, addr).protocol_data_digest)
}