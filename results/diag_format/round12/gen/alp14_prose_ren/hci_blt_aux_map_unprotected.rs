pub open spec fn hci_blt_aux_map_unprotected_spec(vd: Address, lba: Address, index: UInt64, result: Hcicommandreturncode, old_s: S, new_s: S) -> bool {
  ((vd % EXTENT_SIZE(old_s)) != 0 ==> RSI_ERROR_INPUT(result))
  && (!is_physical_address_that_can_be_enrolled(old_s, vd) ==> RSI_ERROR_INPUT(result))
  && (ExtentAt(old_s, vd).vd_state != VD_STATE(old_s) ==> RSI_ERROR_INPUT(result))
  && ((lba % (BLT_ENTRY_SIZE(old_s) as nat)) != 0 ==> RSI_ERROR_INPUT(result))
  && (lba >= pow2(VaultAt(old_s, vd).lba_width as nat) ==> RSI_ERROR_INPUT(result))
  && (is_protected_address_in_vault(old_s, vd, lba) ==> RSI_ERROR_INPUT(result))
  && ((VaultAt(old_s, vd).per_tier_blt_tree_setting == FEATURE_FALSE) ==> RSI_ERROR_INPUT(result))
  && (index == KEEPER_BLT_TREE_PRIMARY ==> RSI_ERROR_INPUT(result))
  && (index > VaultAt(old_s, vd).aux_tier_count ==> RSI_ERROR_INPUT(result))
  && (Bltentryindex(old_s, lba, Bltwalk(old_s, VaultAt(old_s, vd), lba,VaultAt(old_s, vd).blt_level_start as int,KEEPER_BLT_TREE_PRIMARY as int).result.level) == UNASSIGNED_PUB ==> RSI_ERROR_BLT(result))
  && (result == RSI_SUCCESS ==> Bltentry(new_s, Bltwalk(new_s, VaultAt(new_s, vd), lba,VaultAt(new_s, vd).blt_level_start as int,index as int).result.blt_addr, Bltentryindex(new_s, lba, Bltwalk(new_s, VaultAt(new_s, vd), lba,VaultAt(new_s, vd).blt_level_start as int,index as int).result.level) == Bltentry(new_s, Bltwalk(new_s, VaultAt(new_s, vd), lba,VaultAt(new_s, vd).blt_level_start as int,KEEPER_BLT_TREE_PRIMARY as int).result.blt_addr, Bltentryindex(new_s, lba, Bltwalk(new_s, VaultAt(new_s, vd), lba,VaultAt(new_s, vd).blt_level_start as int,KEEPER_BLT_TREE_PRIMARY as int).result.level)))
  && (result == RSI_SUCCESS ==> Bltentry(new_s, Bltwalk(new_s, VaultAt(new_s, vd), lba,VaultAt(new_s, vd).blt_level_start as int,index as int).result.blt_addr, Bltentryindex(new_s, lba, Bltwalk(new_s, VaultAt(new_s, vd), lba,VaultAt(new_s, vd).blt_level_start as int,index as int).result.level).mem_attr == Bltentry(new_s, Bltwalk(new_s, VaultAt(new_s, vd), lba,VaultAt(new_s, vd).blt_level_start as int,KEEPER_BLT_TREE_PRIMARY as int).result.blt_addr, Bltentryindex(new_s, lba, Bltwalk(new_s, VaultAt(new_s, vd), lba,VaultAt(new_s, vd).blt_level_start as int,KEEPER_BLT_TREE_PRIMARY as int).result.level).mem_attr)
  && (result == RSI_SUCCESS ==> Bltentry(new_s, Bltwalk(new_s, VaultAt(new_s, vd), lba,VaultAt(new_s, vd).blt_level_start as int,index as int).result.blt_addr, Bltentryindex(new_s, lba, Bltwalk(new_s, VaultAt(new_s, vd), lba,VaultAt(new_s, vd).blt_level_start as int,index as int).result.level).stage_2_access_perms == Bltentry(new_s, Bltwalk(new_s, VaultAt(new_s, vd), lba,VaultAt(new_s, vd).blt_level_start as int,KEEPER_BLT_TREE_PRIMARY as int).result.blt_addr, Bltentryindex(new_s, lba, Bltwalk(new_s, VaultAt(new_s, vd), lba,VaultAt(new_s, vd).blt_level_start as int,KEEPER_BLT_TREE_PRIMARY as int).result.level).stage_2_access_perms)
  && (result == RSI_SUCCESS ==> Bltentry(new_s, Bltwalk(new_s, VaultAt(new_s, vd), lba,VaultAt(new_s, vd).blt_level_start as int,index as int).result.blt_addr, Bltentryindex(new_s, lba, Bltwalk(new_s, VaultAt(new_s, vd), lba,VaultAt(new_s, vd).blt_level_start as int,index as int).result.level).output_addr == Bltentry(new_s, Bltwalk(new_s, VaultAt(new_s, vd), lba,VaultAt(new_s, vd).blt_level_start as int,KEEPER_BLT_TREE_PRIMARY as int).result.blt_addr, Bltentryindex(new_s, lba, Bltwalk(new_s, VaultAt(new_s, vd), lba,VaultAt(new_s, vd).blt_level_start as int,KEEPER_BLT_TREE_PRIMARY as int).result.level).output_addr)
  && ((!( (vd % EXTENT_SIZE(old_s)) != 0 &&
         !is_physical_address_that_can_be_enrolled(old_s, vd) &&
         ExtentAt(old_s, vd).vd_state != VD_STATE(old_s) &&
         ((lba % (BLT_ENTRY_SIZE(old_s) as nat)) != 0) &&
         (lba >= pow2(VaultAt(old_s, vd).lba_width as nat)) &&
         is_protected_address_in_vault(old_s, vd, lba) &&
         ((VaultAt(old_s, vd).per_tier_blt_tree_setting == FEATURE_FALSE)) &&
         (index == KEEPER_BLT_TREE_PRIMARY) &&
         (index > VaultAt(old_s, vd).aux_tier_count) &&
         (Bltentryindex(old_s, lba, Bltwalk(old_s, VaultAt(old_s, vd), lba,VaultAt(old_s, vd).blt_level_start as int,KEEPER_BLT_TREE_PRIMARY as int).result.level) == UNASSIGNED_PUB))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> Bltentry(new_s, Bltwalk(new_s, VaultAt(new_s, vd), lba,VaultAt(new_s, vd).blt_level_start as int,index as int).result.blt_addr, Bltentryindex(new_s, lba, Bltwalk(new_s, VaultAt(new_s, vd), lba,VaultAt(new_s, vd).blt_level_start as int,index as int).result.level) == Bltentry(old_s, Bltwalk(old_s, VaultAt(old_s, vd), lba,VaultAt(old_s, vd).blt_level_start as int,index as int).result.blt_addr, Bltentryindex(old_s, lba, Bltwalk(old_s, VaultAt(old_s, vd), lba,VaultAt(old_s, vd).blt_level_start as int,index as int).result.level)))
  && (result != RSI_SUCCESS
    ==> Bltentry(new_s, Bltwalk(new_s, VaultAt(new_s, vd), lba,VaultAt(new_s, vd).blt_level_start as int,index as int).result.blt_addr, Bltentryindex(new_s, lba, Bltwalk(new_s, VaultAt(new_s, vd), lba,VaultAt(new_s, vd).blt_level_start as int,index as int).result.level).mem_attr == Bltentry(old_s, Bltwalk(old_s, VaultAt(old_s, vd), lba,VaultAt(old_s, vd).blt_level_start as int,index as int).result.blt_addr, Bltentryindex(old_s, lba, Bltwalk(old_s, VaultAt(old_s, vd), lba,VaultAt(old_s, vd).blt_level_start as int,index as int).result.level).mem_attr))
  && (result != RSI_SUCCESS
    ==> Bltentry(new_s, Bltwalk(new_s, VaultAt(new_s, vd), lba,VaultAt(new_s, vd).blt_level_start as int,index as int).result.blt_addr, Bltentryindex(new_s, lba, Bltwalk(new_s, VaultAt(new_s, vd), lba,VaultAt(new_s, vd).blt_level_start as int,index as int).result.level).stage_2_access_perms == Bltentry(old_s, Bltwalk(old_s, VaultAt(old_s, vd), lba,VaultAt(old_s, vd).blt_level_start as int,index as int).result.blt_addr, Bltentryindex(old_s, lba, Bltwalk(old_s, VaultAt(old_s, vd), lba,VaultAt(old_s, vd).blt_level_start as int,index as int).result.level).stage_2_access_perms))
  && (result != RSI_SUCCESS
    ==> Bltentry(new_s, Bltwalk(new_s, VaultAt(new_s, vd), lba,VaultAt(new_s, vd).blt_level_start as int,index as int).result.blt_addr, Bltentryindex(new_s, lba, Bltwalk(new_s, VaultAt(new_s, vd), lba,VaultAt(new_s, vd).blt_level_start as int,index as int).result.level).output_addr == Bltentry(old_s, Bltwalk(old_s, VaultAt(old_s, vd), lba,VaultAt(old_s, vd).blt_level_start as int,index as int).result.blt_addr, Bltentryindex(old_s, lba, Bltwalk(old_s, VaultAt(old_s, vd), lba,VaultAt(old_s, vd).blt_level_start as int,index as int).result.level).output_addr))
  && (result == RSI_SUCCESS
    ==> VaultAt(new_s, vd).blt_level_start == VaultAt(old_s, vd).blt_level_start)
  && (result == RSI_SUCCESS
    ==> VaultAt(new_s, vd).blt_level_start == VaultAt(old_s, vd).blt_level_start)
}