pub open spec fn hci_blt_create_spec(vd: PhysicalAddress, blt: PhysicalAddress, lba: UInt64, level: int, result: Result<(), HciStatusCode>, old_s: S, new_s: S) -> bool {
  ((vd % ExtentSize(old_s) != 0) ==> result == HCI_ERROR_INPUT)
  && (!is_enrollable_physical_address(old_s, vd) ==> result == HCI_ERROR_INPUT)
  && (ExtentAt(old_s, vd).state != VD_STATE ==> result == HCI_ERROR_INPUT)
  && (!is_valid_blt_level_for_vault(old_s, level) || level == VaultAt(old_s, 0).rtt_level_start) ==> result == HCI_ERROR_INPUT)
  && ((lba % (ExtentSize(old_s) * pow2(64 - (VaultAt(old_s, 0).lba_width as nat))) != 0) ==> result == HCI_ERROR_INPUT)
  && (lba >= pow2(VaultAt(old_s, 0).lba_width)) ==> result == HCI_ERROR_INPUT)
  && ((blt % ExtentSize(old_s) != 0) ==> result == HCI_ERROR_INPUT)
  && (!is_enrollable_physical_address_in_dram(old_s, blt) ==> result == HCI_ERROR_INPUT)
  && (ExtentAt(old_s, blt).state != ENROLLED ==> result == HCI_ERROR_INPUT)
  && ((!(is_valid_blt_level_for_vault(old_s, level) || level == VaultAt(old_s, 0).rtt_level_start) &&
       is_enrollable_physical_address(old_s, vd) &&
       !(ExtentAt(old_s, vd).state != VD_STATE) &&
       (lba < pow2(VaultAt(old_s, 0).lba_width)))
    ==> result == HCI_ERROR_BLT)
  && (result == HCI_ERROR_BLT ==> result == HCI_ERROR_BLT)
  && (result == HCI_ERROR_BLT ==> result == HCI_ERROR_BLT)
  && (result == HCI_SUCCESS ==> ExtentAt(new_s, blt).state == BLT)
  && (result == HCI_SUCCESS ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == TABLE)
  && (result == HCI_SUCCESS ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == blt)
  && ((!(result == HCI_ERROR_INPUT &&
        (vd % ExtentSize(old_s) != 0)) &&
       !(result == HCI_ERROR_INPUT &&
        !is_enrollable_physical_address(old_s, vd)) &&
       !(result == HCI_ERROR_INPUT &&
        (ExtentAt(old_s, vd).state != VD_STATE)) &&
       !(result == HCI_ERROR_INPUT &&
        (!is_valid_blt_level_for_vault(old_s, level) || level == VaultAt(old_s, 0).rtt_level_start)) &&
       !(result == HCI_ERROR_INPUT &&
        ((lba % (ExtentSize(old_s) * pow2(64 - (VaultAt(old_s, 0).lba_width as nat))) != 0)) &&
       !(result == HCI_ERROR_INPUT &&
        (lba >= pow2(VaultAt(old_s, 0).lba_width))) &&
       !(result == HCI_ERROR_INPUT &&
        ((blt % ExtentSize(old_s) != 0)) &&
       !(result == HCI_ERROR_INPUT &&
        !is_enrollable_physical_address_in_dram(old_s, blt)) &&
       !(result == HCI_ERROR_INPUT &&
        (ExtentAt(old_s, blt).state != ENROLLED)))
    ==> ExtentAt(new_s, blt).state == ExtentAt(old_s, blt).state)
  && (result == HCI_SUCCESS &&
       BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == TABLE
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).state)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).blt)
  && (!(result == HCI_SUCCESS &&
       (BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == TABLE))
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).blt)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == blt)
  && (result != HCI_SUCCESS
    ==> ExtentAt(new_s, blt).state == ExtentAt(old_s, blt).state)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).state)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).blt)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == TABLE)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == blt)
  && ((!(result == HCI_ERROR_INPUT &&
        (vd % ExtentSize(old_s) != 0)) &&
       !(result == HCI_ERROR_INPUT &&
        !is_enrollable_physical_address(old_s, vd)) &&
       !(result == HCI_ERROR_INPUT &&
        (ExtentAt(old_s, vd).state != VD_STATE)) &&
       !(result == HCI_ERROR_INPUT &&
        (!is_valid_blt_level_for_vault(old_s, level) || level == VaultAt(old_s, 0).rtt_level_start)) &&
       !(result == HCI_ERROR_INPUT &&
        ((lba % (ExtentSize(old_s) * pow2(64 - (VaultAt(old_s, 0).lba_width as nat))) != 0)) &&
       !(result == HCI_ERROR_INPUT &&
        (lba >= pow2(VaultAt(old_s, 0).lba_width))) &&
       !(result == HCI_ERROR_INPUT &&
        ((blt % ExtentSize(old_s) != 0)) &&
       !(result == HCI_ERROR_INPUT &&
        !is_enrollable_physical_address_in_dram(old_s, blt)) &&
       !(result == HCI_ERROR_INPUT &&
        (ExtentAt(old_s, blt).state != ENROLLED)))
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).state)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == TABLE)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == blt)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).state)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).blt)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == TABLE)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == blt)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).state)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).blt)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == TABLE)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == blt)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).state)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).blt)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == TABLE)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == blt)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).state)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).blt)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == TABLE)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == blt)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).state)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).blt)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == TABLE)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == blt)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).state)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).blt)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == TABLE)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == blt)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).state)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).blt)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == TABLE)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == blt)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).state)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).blt)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == TABLE)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == blt)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).state)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).blt)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == TABLE)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == blt)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).state)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).blt)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == TABLE)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == blt)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).state)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).blt)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == TABLE)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == blt)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).state)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).blt)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == TABLE)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == blt)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).state)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).blt)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == TABLE)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == blt)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).state)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).blt)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == TABLE)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == blt)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).state)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).blt)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == TABLE)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == blt)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).state)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).blt)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == TABLE)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == blt)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).state)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).blt)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == TABLE)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == blt)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).state)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).blt)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == TABLE)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == blt)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).state)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).blt)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == TABLE)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == blt)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).state)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).blt)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == TABLE)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == blt)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).state)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).blt)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == TABLE)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == blt)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).state)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).blt)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == TABLE)
  && (result == HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).blt == blt)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, lba).state == BLTEntry(old_s, VaultAt(old_s, 0).primary_blt_tree, lba).state)
  && (result != HCI_SUCCESS
    ==> BLTEntry(new_s, VaultAt(new_s, 0).primary_blt_tree, l