pub open spec fn rsi_rdev_get_info_spec(vdev_id: Bits64, inst_id: UInt64, addr: Address, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (CurrentRealm(old_s).feat_da != FEATURE_TRUE ==> result == RSI_ERROR_STATE)
  && (!VdevIdsAreValid(old_s, CurrentRealm(old_s), vdev_id, inst_id as int) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsGranuleAligned(old_s, addr) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsProtected(old_s, addr, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
  && (RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == EMPTY ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> Equal(RsiDevInfoAt(new_s, addr).hash_algo, PdevAt(new_s, VdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).pdev).hash_algo))
  && (result == RSI_SUCCESS ==> Equal(RsiDevInfoAt(new_s, addr).flags.p2p, PdevAt(new_s, VdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).pdev).p2p_enabled))
  && ((!(CurrentRealm(old_s).feat_da != FEATURE_TRUE) &&
       VdevIdsAreValid(old_s, CurrentRealm(old_s), vdev_id, inst_id as int) &&
       AddrIsGranuleAligned(old_s, addr) &&
       AddrIsProtected(old_s, addr, CurrentRealm(old_s)) &&
       !(RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == EMPTY))
    ==> result == RSI_SUCCESS)
}
