pub open spec fn rsi_realm_config_spec(addr: Address, result: RsiCommandReturnCode, cfg: RsiRealmConfig, old_s: S, new_s: S) -> bool {
  (!AddrIsRsiGranuleAligned(old_s, addr) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsProtected(old_s, addr, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
  && (RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RIPAS_EMPTY ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> cfg.ipa_width == CurrentRealm(new_s).ipa_width)
  && (result == RSI_SUCCESS ==> Equal(cfg.hash_algo, CurrentRealm(new_s).hash_algo))
  && (result == RSI_SUCCESS ==> cfg.num_aux_planes == CurrentRealm(new_s).num_aux_planes)
  && (result == RSI_SUCCESS ==> cfg.ats_plane == CurrentRealm(new_s).ats_plane)
  && ((!(AddrIsRsiGranuleAligned(old_s, addr)) &&
       AddrIsProtected(old_s, addr, CurrentRealm(old_s)) &&
       !(RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RIPAS_EMPTY))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> CurrentRealm(new_s).ipa_width == CurrentRealm(old_s).ipa_width)
  && (result != RSI_SUCCESS
    ==> CurrentRealm(new_s).hash_algo == CurrentRealm(old_s).hash_algo)
  && (result != RSI_SUCCESS
    ==> CurrentRealm(new_s).num_aux_planes == CurrentRealm(old_s).num_aux_planes)
  && (result != RSI_SUCCESS
    ==> CurrentRealm(new_s).ats_plane == CurrentRealm(old_s).ats_plane)
}