pub open spec fn hci_blt_aux_unmap_unprotected_spec(vd: UInt64, lba: UInt64, index: UInt64, result: Result<(), RsiCommandReturnCode>, top: UInt64, old_s: S, new_s: S) -> bool {
  ((vd % EXTENT_SIZE) != 0 ==> result == RSI_ERROR_INPUT)
  && (!is_physical_address_that_can_be_enrolled(vd, old_s) ==> result == RSI_ERROR_INPUT)
  && (ExtentAt(old_s, vd).state != VD_STATE ==> result == RSI_ERROR_INPUT)
  && (!is_lba_aligned_to_size_of_address_range_that_one_blt_entry_maps_at_the_vaults_starting_blt_level(old_s, lba) ==> result == RSI_ERROR_INPUT)
  && ((lba >= 2^(VaultAt(old_s, vd).lba_width as nat)) ==> result == RSI_ERROR_INPUT)
  && (is_protected_lba_of_the_vault(old_s, vd, lba) ==> result == RSI_ERROR_INPUT)
  && (VaultAt(old_s, vd).blt_tree_per_tier == FEATURE_FALSE ==> result == RSI_ERROR_INPUT)
  && (index == KEEPER_BLT_TREE_PRIMARY ==> result == RSI_ERROR_INPUT)
  && (index > VaultAt(old_s, vd).number_of_auxiliary_tiers ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> BLTEntryAt(new_s, VaultAt(new_s, vd).starting_blt_level, lba, index as int).state == UNASSIGNED_PUB)
  && (result == RSI_SUCCESS ==> top == top_lba_of_non_live_blt_entries_that_the_blt_walk_produces(new_s, VaultAt(new_s, vd).starting_blt_level, lba, index as int))
  && ((!( (vd % EXTENT_SIZE) != 0) &&
       is_physical_address_that_can_be_enrolled(vd, old_s) &&
       !(ExtentAt(old_s, vd).state != VD_STATE) &&
       is_lba_aligned_to_size_of_address_range_that_one_blt_entry_maps_at_the_vaults_starting_blt_level(old_s, lba) &&
       !((lba >= 2^(VaultAt(old_s, vd).lba_width as nat))) &&
       !(is_protected_lba_of_the_vault(old_s, vd, lba)) &&
       !(VaultAt(old_s, vd).blt_tree_per_tier == FEATURE_FALSE) &&
       !(index == KEEPER_BLT_TREE_PRIMARY) &&
       !(index > VaultAt(old_s, vd).number_of_auxiliary_tiers))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> BLTEntryAt(new_s, VaultAt(new_s, vd).starting_blt_level, lba, index as int).state == BLTEntryAt(old_s, VaultAt(old_s, vd).starting_blt_level, lba, index as int).state)
}