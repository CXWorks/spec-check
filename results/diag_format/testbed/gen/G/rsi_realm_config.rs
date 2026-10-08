pub open spec fn rsi_realm_config_spec(addr: Address, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (AddrIsAligned(old_s, addr, RMM_GRANULE_SIZE_ORDER as int) ==> result == RSI_SUCCESS)
  (AddrIsProtected(old_s, addr, CurrentRealm(old_s)) ==> result == RSI_SUCCESS)
  (RttWalk_(old_s, CurrentRealm(old_s).rtt_base[0], addr,RMM_RTT_PAGE_LEVEL as int).rtte.ripas == RMM_RTT_ENTRY_STATE::UNASSIGNED ==> result == RSI_SUCCESS)
  (result == RSI_ERROR_INPUT ==> (AddrIsAligned(old_s, addr, RMM_GRANULE_SIZE_ORDER as int) && AddrIsProtected(old_s, addr, CurrentRealm(old_s)) && !(RttWalk_(old_s, CurrentRealm(old_s).rtt_base[0], addr,RMM_RTT_PAGE_LEVEL as int).rtte.ripas == RMM_RTT_ENTRY_STATE::UNASSIGNED)))
  (result != RSI_SUCCESS ==> CurrentRealm(new_s).ipa_width == CurrentRealm(old_s).ipa_width)
  (result != RSI_SUCCESS ==> CurrentRealm(new_s).hash_algo == CurrentRealm(old_s).hash_algo)
  (result != RSI_SUCCESS ==> CurrentRealm(new_s).num_aux_planes == CurrentRealm(old_s).num_aux_planes)
  (result != RSI_SUCCESS ==> CurrentRealm(new_s).ats_plane == CurrentRealm(old_s).ats_plane)
}