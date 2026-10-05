pub open spec fn hci_blt_aux_map_unprotected_spec(vd: UInt64, lba: UInt64, index: UInt64, result: Result<(), HciStatusCode>, old_s: S, new_s: S) -> bool {
  ((vd % EXTENT_SIZE) != 0 ==> result == HCI_ERROR_INPUT)
  && (!is_physical_address_that_can_be_enrolled(old_s, vd) ==> result == HCI_ERROR_INPUT)
  && (ExtentAt(old_s, vd).state != VD_STATE ==> result == HCI_ERROR_INPUT)
  && ((lba % (BLT_ENTRY_SIZE_at_starting_blt_level(RealmAt(old_s, 0).lba_width) as nat)) != 0 ==> result == HCI_ERROR_INPUT)
  && (lba >= 2^(RealmAt(old_s, 0).lba_width) ==> result == HCI_ERROR_INPUT)
  && (is_protected_address_in_vault(old_s, vd, lba) ==> result == HCI_ERROR_INPUT)
  && ((!(RealmAt(old_s, 0).per_tier_blt_tree == FEATURE_FALSE) &&
       index != KEEPER_BLT_TREE_PRIMARY &&
       !(index > number_of_auxiliary_tiers_in_vault(old_s)))
    ==> result == HCI_ERROR_INPUT)
  && (result == HCI_ERROR_BLT ==> result == HCI_ERROR_BLT)
  && (result == HCI_SUCCESS
    ==> BltEntryAt(new_s, 0, lba, index as int).state == BltEntryAt(new_s, 0, lba, 0 as int).state)
  && (result == HCI_SUCCESS
    ==> BltEntryAt(new_s, 0, lba, index as int).memory_attributes == BltEntryAt(new_s, 0, lba, 0 as int).memory_attributes)
  && (result == HCI_SUCCESS
    ==> BltEntryAt(new_s, 0, lba, index as int).stage_2_access_permissions == BltEntryAt(new_s, 0, lba, 0 as int).stage_2_access_permissions)
  && (result == HCI_SUCCESS
    ==> BltEntryAt(new_s, 0, lba, index as int).output_address == BltEntryAt(new_s, 0, lba, 0 as int).output_address)
  && ((!( (vd % EXTENT_SIZE) != 0) &&
       is_physical_address_that_can_be_enrolled(old_s, vd) &&
       !(ExtentAt(old_s, vd).state != VD_STATE) &&
       !((lba % (BLT_ENTRY_SIZE_at_starting_blt_level(RealmAt(old_s, 0).lba_width) as nat)) != 0) &&
       !(lba >= 2^(RealmAt(old_s, 0).lba_width)) &&
       !(is_protected_address_in_vault(old_s, vd, lba)) &&
       !((!(RealmAt(old_s, 0).per_tier_blt_tree == FEATURE_FALSE) &&
          index != KEEPER_BLT_TREE_PRIMARY &&
          !(index > number_of_auxiliary_tiers_in_vault(old_s))))))
    ==> result == HCI_SUCCESS)
  && (result != HCI_SUCCESS
    ==> BltEntryAt(new_s, 0, lba, index as int).state == BltEntryAt(old_s, 0, lba, index as int).state)
  && (result != HCI_SUCCESS
    ==> BltEntryAt(new_s, 0, lba, index as int).memory_attributes == BltEntryAt(old_s, 0, lba, index as int).memory_attributes)
  && (result != HCI_SUCCESS
    ==> BltEntryAt(new_s, 0, lba, index as int).stage_2_access_permissions == BltEntryAt(old_s, 0, lba, index as int).stage_2_access_permissions)
  && (result != HCI_SUCCESS
    ==> BltEntryAt(new_s, 0, lba, index as int).output_address == BltEntryAt(old_s, 0, lba, index as int).output_address)
}