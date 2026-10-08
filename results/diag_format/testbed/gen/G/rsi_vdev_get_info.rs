pub open spec fn rsi_vdev_get_info_spec(vdev_id: Bits64, addr: Address, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (CurrentRealm(old_s).feat_da == FEATURE_FALSE ==> result == RSI_ERROR_STATE)
  && (VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).vdev_state == VDEV_NEW ==> result == RSI_ERROR_INPUT)
  && ((addr) % 512 != 0 ==> result == RSI_ERROR_INPUT)
  && (AddrIsWithin(old_s, addr, CurrentRealm(old_s).rtt_base[0], CurrentRealm(old_s).rtt_base[0] + 16 * CurrentRealm(old_s).rtt_num_start) ==> result == RSI_ERROR_INPUT)
  && (RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == EMPTY ==> result == RSI_ERROR_INPUT)
  && ((!(CurrentRealm(old_s).feat_da == FEATURE_FALSE) &&
       !(VdevFromVdevId(old_s, CurrentRealm(old_s), vdev_id).vdev_state == VDEV_NEW) &&
       ((addr) % 512 == 0) &&
       !(AddrIsWithin(old_s, addr, CurrentRealm(old_s).rtt_base[0], CurrentRealm(old_s).rtt_base[0] + 16 * CurrentRealm(old_s).rtt_num_start)) &&
       !(RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == EMPTY))
    ==> result == RSI_SUCCESS)
}