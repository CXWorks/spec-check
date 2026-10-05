pub open spec fn rsi_realm_config_spec(addr: Address, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == RSI_ERROR_INPUT && (addr % GRANULE_SIZE != 0))
  && (result == RSI_ERROR_INPUT && !IsProtectedIpa(old_s, addr))
  && (result == RSI_ERROR_INPUT && RealmAt(old_s, CurrentRealm(old_s)).ripas[addr / GRANULE_SIZE as int] == EMPTY)
  && ((!(addr % GRANULE_SIZE != 0) &&
       IsProtectedIpa(old_s, addr) &&
       !(RealmAt(old_s, CurrentRealm(old_s)).ripas[addr / GRANULE_SIZE as int] == EMPTY))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> RealmAt(new_s, CurrentRealm(new_s)).ipa_width == RealmAt(old_s, CurrentRealm(old_s)).ipa_width)
  && (result != RSI_SUCCESS
    ==> RealmAt(new_s, CurrentRealm(new_s)).hash_algo == RealmAt(old_s, CurrentRealm(old_s)).hash_algo)
  && (result != RSI_SUCCESS
    ==> RealmAt(new_s, CurrentRealm(new_s)).num_aux_planes == RealmAt(old_s, CurrentRealm(old_s)).num_aux_planes)
  && (result != RSI_SUCCESS
    ==> RealmAt(new_s, CurrentRealm(new_s)).ats_plane == RealmAt(old_s, CurrentRealm(old_s)).ats_plane)
  && (result == RSI_SUCCESS
    ==> RealmAt(new_s, CurrentRealm(new_s)).ipa_width == RealmAt(old_s, CurrentRealm(old_s)).ipa_width)
  && (result == RSI_SUCCESS
    ==> RealmAt(new_s, CurrentRealm(new_s)).hash_algo == RealmAt(old_s, CurrentRealm(old_s)).hash_algo)
  && (result == RSI_SUCCESS
    ==> RealmAt(new_s, CurrentRealm(new_s)).num_aux_planes == RealmAt(old_s, CurrentRealm(old_s)).num_aux_planes)
  && (result == RSI_SUCCESS
    ==> RealmAt(new_s, CurrentRealm(new_s)).ats_plane == RealmAt(old_s, CurrentRealm(old_s)).ats_plane)
  && (!(result == RSI_ERROR_INPUT && (addr % GRANULE_SIZE != 0)) &&
       !(result == RSI_ERROR_INPUT && !IsProtectedIpa(old_s, addr)) &&
       !(result == RSI_ERROR_INPUT && RealmAt(old_s, CurrentRealm(old_s)).ripas[addr / GRANULE_SIZE as int] == EMPTY)
    ==> result == RSI_SUCCESS)
}