pub open spec fn vci_vault_config_spec(addr: Address, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  ((addr % EXTENT_SIZE(old_s) != 0) ==> result == VCI_ERROR_INPUT)
  && (!IsProtectedLba(old_s, addr) ==> result == VCI_ERROR_INPUT)
  && (LBAMode(old_s, addr) == EMPTY ==> result == VCI_ERROR_INPUT)
  && (result == VCI_SUCCESS ==> VaultConfiguration(new_s).lba_width == LbaWidth(new_s))
  && (result == VCI_SUCCESS ==> VaultConfiguration(new_s).hash_algo == VaultHashAlgorithm(new_s))
  && (result == VCI_SUCCESS ==> VaultConfiguration(new_s).num_aux_tiers == NumAuxTiers(new_s))
  && (result == VCI_SUCCESS ==> VaultConfiguration(new_s).xlc_tier == XlcTier(new_s))
  && ((!(addr % EXTENT_SIZE(old_s) != 0) &&
       IsProtectedLba(old_s, addr) &&
       !(LBAMode(old_s, addr) == EMPTY))
    ==> result == VCI_SUCCESS)
  && (result != VCI_SUCCESS
    ==> VaultConfiguration(new_s).lba_width == VaultConfiguration(old_s).lba_width)
  && (result != VCI_SUCCESS
    ==> VaultConfiguration(new_s).hash_algo == VaultConfiguration(old_s).hash_algo)
  && (result != VCI_SUCCESS
    ==> VaultConfiguration(new_s).num_aux_tiers == VaultConfiguration(old_s).num_aux_tiers)
  && (result != VCI_SUCCESS
    ==> VaultConfiguration(new_s).xlc_tier == VaultConfiguration(old_s).xlc_tier)
}