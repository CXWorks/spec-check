pub open spec fn hci_blt_aux_unmap_unprotected_spec(vd: Address, lba: Address, index: UInt64, result: Hcicommandreturncode, top: Address, old_s: S, new_s: S) -> bool {
  ((vd % ExtentSize(old_s) != 0) ==> result == HCI_ERROR_INPUT)
  && ((!(is_physical_address_that_can_be_enrolled(old_s, vd)) ) ==> result == HCI_ERROR_INPUT)
  && (ExtentAt(old_s, vd).state != VD_STATE_EXTENT_IN_VD ==> result == HCI_ERROR_INPUT)
  && ((lba % (BLTEntrySize(old_s) as nat)) != 0) ==> result == HCI_ERROR_INPUT)
  && ((lba >= pow2(Vaultat(old_s, vd).lba_width as nat)) ==> result == HCI_ERROR_INPUT)
  && (Vaultat(old_s, vd).is_protected_lba(lba) ==> result == HCI_ERROR_INPUT)
  && (Vaultat(old_s, vd).blt_tree_per_tier == FEATURE_FALSE ==> result == HCI_ERROR_INPUT)
  && (index == KEEPER_BLT_TREE_PRIMARY ==> result == HCI_ERROR_INPUT)
  && (index > Vaultat(old_s, vd).num_auxiliary_tiers ==> result == HCI_ERROR_INPUT)
  && (result == HCI_SUCCESS ==> Bltentry(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba,new_s.blt_level_start,index as int).blt_addr, Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba,new_s.blt_level_start,index as int).level)) == UNASSIGNED_PUB)
  && (result == HCI_SUCCESS ==> top == Bltskipnonliveentries(new_s, Bltat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba,new_s.blt_level_start,index as int).blt_addr),Bltwalk(new_s, Vaultat(new_s, vd), lba,new_s.blt_level_start,index as int).level,lba))
  && ((!( (vd % ExtentSize(old_s) != 0) &&
         (!(is_physical_address_that_can_be_enrolled(old_s, vd)) ) &&
         (ExtentAt(old_s, vd).state != VD_STATE_EXTENT_IN_VD) &&
         ((lba % (BLTEntrySize(old_s) as nat)) != 0) &&
         ((lba >= pow2(Vaultat(old_s, vd).lba_width as nat)) ) &&
         (Vaultat(old_s, vd).is_protected_lba(lba)) &&
         (Vaultat(old_s, vd).blt_tree_per_tier == FEATURE_FALSE) &&
         (index == KEEPER_BLT_TREE_PRIMARY) &&
         (index > Vaultat(old_s, vd).num_auxiliary_tiers)
       ))
    ==> result == HCI_SUCCESS)
  && (result != HCI_SUCCESS
    ==> Bltentry(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba,new_s.blt_level_start,index as int).blt_addr, Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba,new_s.blt_level_start,index as int).level)) == Bltentry(old_s, Bltwalk(old_s, Vaultat(old_s, vd), lba,old_s.blt_level_start,index as int).blt_addr, Bltentryindex(old_s, lba, Bltwalk(old_s, Vaultat(old_s, vd), lba,old_s.blt_level_start,index as int).level)))
  && (result != HCI_SUCCESS
    ==> top == Bltskipnonliveentries(new_s, Bltat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba,new_s.blt_level_start,index as int).blt_addr),Bltwalk(new_s, Vaultat(new_s, vd), lba,new_s.blt_level_start,index as int).level,lba))
}