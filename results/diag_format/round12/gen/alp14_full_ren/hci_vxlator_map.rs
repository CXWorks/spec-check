pub open spec fn hci_vxlator_map_spec(vd: PhysicalAddress, vxlator_ptr: PhysicalAddress, lba: UInt64, level: int, result: Result<(), HciStatusCode>, old_s: S, new_s: S) -> bool {
  (result == HCI_ERROR_NOT_SUPPORTED ==> true)
  && (vd % ExtentSize(old_s) != 0 ==> result == HCI_ERROR_INPUT)
  && (!IsEnrollableAddress(old_s, vd) ==> result == HCI_ERROR_INPUT)
  && (ExtentAt(old_s, vd).state != VD_STATE ==> result == HCI_ERROR_INPUT)
  && (result == HCI_ERROR_INPUT ==> result == HCI_ERROR_INPUT)
  && (result == HCI_ERROR_INPUT ==> result == HCI_ERROR_INPUT)
  && (result == HCI_ERROR_INPUT ==> result == HCI_ERROR_INPUT)
  && (result == HCI_ERROR_INPUT ==> result == HCI_ERROR_INPUT)
  && (result == HCI_ERROR_INPUT ==> result == HCI_ERROR_INPUT)
  && (result == HCI_ERROR_INPUT ==> result == HCI_ERROR_INPUT)
  && (result == HCI_ERROR_INPUT ==> result == HCI_ERROR_INPUT)
  && (result == HCI_ERROR_INPUT ==> result == HCI_ERROR_INPUT)
  && (result == HCI_ERROR_INPUT ==> result == HCI_ERROR_INPUT)
  && (result == HCI_ERROR_INPUT ==> result == HCI_ERROR_INPUT)
  && (result == HCI_ERROR_BLT ==> result == HCI_ERROR_BLT)
  && (result == HCI_ERROR_BLT ==> result == HCI_ERROR_BLT)
  && (result == HCI_ERROR_BLT ==> result == HCI_ERROR_BLT)
  && (result == HCI_ERROR_BLT ==> result == HCI_ERROR_BLT)
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result == HCI_ERROR_INPUT &&
       result == HCI_ERROR_INPUT &&
       result == HCI_ERROR_INPUT &&
       result == HCI_ERROR_INPUT &&
       result == HCI_ERROR_INPUT &&
       result == HCI_ERROR_INPUT &&
       result == HCI_ERROR_INPUT &&
       result == HCI_ERROR_INPUT &&
       result == HCI_ERROR_INPUT &&
       result == HCI_ERROR_INPUT &&
       result == HCI_ERROR_INPUT &&
       result == HCI_ERROR_INPUT &&
       result == HCI_ERROR_INPUT &&
       result != HCI_ERROR_BLT &&
       result != HCI_ERROR_BLT &&
       result != HCI_ERROR_BLT &&
       result != HCI_ERROR_BLT)
    ==> BLTEntry(new_s, BLTWalk(new_s, VaultAt(new_s, VDAt(new_s, vd)).vault, lba as int, level as int).entry_idx).state == ASSIGNED_VXLATOR)
  && (result.is_Ok()
    ==> BLTEntry(new_s, BLTWalk(new_s, VaultAt(new_s, VDAt(new_s, vd)).vault, lba as int, level as int).entry_idx).address == vxlator_ptr)
  && ((!(result == HCI_ERROR_NOT_SUPPORTED) &&
       result == HCI_ERROR_INPUT &&
       result == HCI_ERROR_INPUT &&
       result == HCI_ERROR_INPUT &&
       result == HCI_ERROR_INPUT &&
       result == HCI_ERROR_INPUT &&
       result == HCI_ERROR_INPUT &&
       result == HCI_ERROR_INPUT &&
       result == HCI_ERROR_INPUT &&
       result == HCI_ERROR_INPUT &&
       result == HCI_ERROR_INPUT &&
       result == HCI_ERROR_INPUT &&
       result == HCI_ERROR_INPUT &&
       result == HCI_ERROR_INPUT &&
       result != HCI_ERROR_BLT &&
       result != HCI_ERROR_BLT &&
       result != HCI_ERROR_BLT &&
       result != HCI_ERROR_BLT)
    ==> BLTEntry(new_s, BLTWalk(new_s, VaultAt(new_s, VDAt(new_s, vd)).vault, lba as int, level as int).entry_idx).state == BLTEntry(old_s, BLTWalk(old_s, VaultAt(old_s, VDAt(old_s, vd)).vault, lba as int, level as int).entry_idx).state)
  && (result.is_Err()
    ==> BLTEntry(new_s, BLTWalk(new_s, VaultAt(new_s, VDAt(new_s, vd)).vault, lba as int, level as int).entry_idx).address == BLTEntry(old_s, BLTWalk(old_s, VaultAt(old_s, VDAt(old_s, vd)).vault, lba as int, level as int).entry_idx).address)
  && (result.is_Err()
    ==> BLTEntry(new_s, BLTWalk(new_s, VaultAt(new_s, VDAt(new_s, vd)).vault, lba as int, level as int).entry_idx).lbamode == BLTEntry(old_s, BLTWalk(old_s, VaultAt(old_s, VDAt(old_s, vd)).vault, lba as int, level as int).entry_idx).lbamode)
}