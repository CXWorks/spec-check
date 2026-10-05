pub open spec fn rsi_realm_config_spec(addr: Address, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  ((!(addr % GRANULE_SIZE == 0) || CurrentRealm(old_s).ripas[addr / GRANULE_SIZE as int] == EMPTY) ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> RsiRealmConfigAt(new_s).ipa_width == CurrentRealm(new_s).ipa_width)
  && (result == RSI_SUCCESS ==> RsiRealmConfigAt(new_s).hash_algo == CurrentRealm(new_s).hash_algo)
  && (result == RSI_SUCCESS ==> RsiRealmConfigAt(new_s).num_aux_planes == CurrentRealm(new_s).num_aux_planes)
  && (result == RSI_SUCCESS ==> RsiRealmConfigAt(new_s).ats_plane == CurrentRealm(new_s).ats_plane)
  && ((!(addr % GRANULE_SIZE == 0) && CurrentRealm(old_s).ripas[addr / GRANULE_SIZE as int] != EMPTY)
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> RsiRealmConfigAt(new_s).ipa_width == RsiRealmConfigAt(old_s).ipa_width)
  && (result != RSI_SUCCESS
    ==> RsiRealmConfigAt(new_s).hash_algo == RsiRealmConfigAt(old_s).hash_algo)
  && (result != RSI_SUCCESS
    ==> RsiRealmConfigAt(new_s).num_aux_planes == RsiRealmConfigAt(old_s).num_aux_planes)
  && (result != RSI_SUCCESS
    ==> RsiRealmConfigAt(new_s).ats_plane == RsiRealmConfigAt(old_s).ats_plane)
}