pub open spec fn vci_vault_config_spec(addr: Address, result: VciCommandReturnCode, old_s: S, new_s: S) -> bool {
  ((addr % EXTENT_SIZE(old_s) != 0) ==> result == VCI_ERROR_INPUT)
  && (!IsProtectedLba(old_s, Currentvault(old_s), addr) ==> result == VCI_ERROR_INPUT)
  && (Bltwalk(old_s, Currentvault(old_s), addr,KEEPER_BLT_PAGE_LEVEL,KEEPER_BLT_TREE_PRIMARY).LBAMODE == EMPTY ==> result == VCI_ERROR_INPUT)
  && (result == VCI_SUCCESS ==> Vcivaultconfigat(new_s, addr).lba_width == Currentvault(new_s).lba_width)
  && (result == VCI_SUCCESS ==> Vcivaultconfigat(new_s, addr).hash_algo == Currentvault(new_s).hash_algo)
  && (result == VCI_SUCCESS ==> Vcivaultconfigat(new_s, addr).num_aux_tiers == Currentvault(new_s).num_aux_tiers)
  && (result == VCI_SUCCESS ==> Vcivaultconfigat(new_s, addr).xlc_tier == Currentvault(new_s).xlc_tier)
  && ((!(addr % EXTENT_SIZE(old_s) != 0) &&
       IsProtectedLba(old_s, Currentvault(old_s), addr) &&
       !(Bltwalk(old_s, Currentvault(old_s), addr,KEEPER_BLT_PAGE_LEVEL,KEEPER_BLT_TREE_PRIMARY).LBAMODE == EMPTY))
    ==> result == VCI_SUCCESS)
  && (result != VCI_SUCCESS
    ==> Vcivaultconfigat(new_s, addr).lba_width == Vcivaultconfigat(old_s, addr).lba_width)
  && (result != VCI_SUCCESS
    ==> Vcivaultconfigat(new_s, addr).hash_algo == Vcivaultconfigat(old_s, addr).hash_algo)
  && (result != VCI_SUCCESS
    ==> Vcivaultconfigat(new_s, addr).num_aux_tiers == Vcivaultconfigat(old_s, addr).num_aux_tiers)
  && (result != VCI_SUCCESS
    ==> Vcivaultconfigat(new_s, addr).xlc_tier == Vcivaultconfigat(old_s, addr).xlc_tier)
}