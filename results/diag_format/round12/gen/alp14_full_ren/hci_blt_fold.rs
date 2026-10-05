pub open spec fn hci_blt_fold_spec(vd: PhysicalAddress, lba: Address, level: UInt, result: RsiCommandReturnCode, blt: PhysicalAddress, old_s: S, new_s: S) -> bool {
  (is_enrollable_physical_address(old_s, vd) ==> result == RSI_SUCCESS)
  && (extent_at(old_s, vd).vd_state == VD_STATE ==> result == RSI_SUCCESS)
  && (is_valid_blt_level(old_s, vd, level) ==> result == RSI_SUCCESS)
  && (!is_starting_level_of_vault_blt(old_s, vd, level) ==> result == RSI_SUCCESS)
  && (is_aligned_to_blt_entry_size_at_level_minus_one(old_s, lba, level - 1 as int) ==> result == RSI_SUCCESS)
  && (lba < pow2(RealmAt(old_s, vd).lba_width as nat) ==> result == RSI_SUCCESS)
  && (blt_walk_reaches_level_minus_one(old_s, vd, lba, level - 1 as int) ==> result == RSI_SUCCESS)
  && (blt_entry_at_walk_result_is_in_TABLE_state(old_s, vd, lba, level - 1 as int) ==> result == RSI_SUCCESS)
  && (blt_pointed_to_by_entry_is_homogeneous(old_s, vd, lba, level - 1 as int) ==> result == RSI_SUCCESS)
  && (!lba_referenced_by_auxiliary_blt(old_s, vd, lba) ==> result == RSI_SUCCESS)
  && ((!(is_enrollable_physical_address(old_s, vd)) ||
       !(extent_at(old_s, vd).vd_state == VD_STATE))
    ==> result == RSI_ERROR_INPUT)
  && ((!is_valid_blt_level(old_s, vd, level))
    ==> result == RSI_ERROR_INPUT)
  && (is_starting_level_of_vault_blt(old_s, vd, level)
    ==> result == RSI_ERROR_INPUT)
  && (!is_aligned_to_blt_entry_size_at_level_minus_one(old_s, lba, level - 1 as int))
    ==> result == RSI_ERROR_INPUT)
  && (lba >= pow2(RealmAt(old_s, vd).lba_width as nat))
    ==> result == RSI_ERROR_INPUT)
  && (!blt_walk_reaches_level_minus_one(old_s, vd, lba, level - 1 as int))
    ==> result == RSI_ERROR_BLT)
  && (!blt_entry_at_walk_result_is_in_TABLE_state(old_s, vd, lba, level - 1 as int))
    ==> result == RSI_ERROR_BLT)
  && (!blt_pointed_to_by_entry_is_homogeneous(old_s, vd, lba, level - 1 as int))
    ==> result == RSI_ERROR_BLT)
  && (lba_referenced_by_auxiliary_blt(old_s, vd, lba))
    ==> result == RSI_ERROR_BLT)
  && (result == RSI_SUCCESS
    ==> extent_at(new_s, vd).vd_state == ENROLLED)
  && ((result != RSI_SUCCESS)
    ==> extent_at(new_s, vd).vd_state == extent_at(old_s, vd).vd_state)
  && ((result == RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).state == VaultBltAt(old_s, vd, lba).state)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).state == VaultBltAt(old_s, vd, lba).state)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).output_address == VaultBltAt(old_s, vd, lba).output_address)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).output_address == VaultBltAt(old_s, vd, lba).output_address)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).memory_attributes == VaultBltAt(old_s, vd, lba).memory_attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).memory_attributes == VaultBltAt(old_s, vd, lba).memory_attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).stage_2_access_permissions == VaultBltAt(old_s, vd, lba).stage_2_access_permissions)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).stage_2_access_permissions == VaultBltAt(old_s, vd, lba).stage_2_access_permissions)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).LBAMODE == VaultBltAt(old_s, vd, lba).LBAMODE)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).LBAMODE == VaultBltAt(old_s, vd, lba).LBAMODE)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).LBAMODE == VaultBltAt(old_s, vd, lba).LBAMODE)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).LBAMODE == VaultBltAt(old_s, vd, lba).LBAMODE)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && ((result != RSI_SUCCESS)
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd, lba).attributes)
  && (result == RSI_SUCCESS
    ==> VaultBltAt(new_s, vd, lba).attributes == VaultBltAt(old_s, vd,