pub open spec fn hci_data_create_spec(vd: UInt64, data: UInt64, lba: UInt64, src: UInt64, flags: UInt64, result: Result<(), HciStatusCode>, old_s: S, new_s: S) -> bool {
  (src % EXTENT_SIZE == 0 ==> result == HCI_ERROR_INPUT)
  && (ExtentAt(old_s, src).state != EXTENT_ACCESSIBLE ==> result == HCI_ERROR_INPUT)
  && (data % EXTENT_SIZE == 0 ==> result == HCI_ERROR_INPUT)
  && (ExtentAt(old_s, data).state != ENROLLED ==> result == HCI_ERROR_INPUT)
  && (data >= 2^48 && !(VaultAt(old_s, vd).feat_bigaddr == FEATURE_TRUE) ==> result == HCI_ERROR_INPUT)
  && (vd % EXTENT_SIZE == 0 ==> result == HCI_ERROR_INPUT)
  && (ExtentAt(old_s, vd).state != VD_STATE ==> result == HCI_ERROR_INPUT)
  && (lba % EXTENT_SIZE == 0 ==> result == HCI_ERROR_INPUT)
  && (!VaultAt(old_s, vd).lba_protected.contains(lba) ==> result == HCI_ERROR_INPUT)
  && (VaultAt(old_s, vd).state != VAULT_NEW ==> result == HCI_ERROR_VAULT)
  && (RttWalk(old_s, vd, KEEPER_BLT_PAGE_LEVEL as int).state != UNASSIGNED ==> result == HCI_ERROR_BLT(RttWalk(new_s, vd, KEEPER_BLT_PAGE_LEVEL as int).level as int))
  && (result == HCI_SUCCESS ==> ExtentAt(new_s, data).state == DATA)
  && (result == HCI_SUCCESS ==> BltEntry(new_s, vd, lba).state == ASSIGNED)
  && (result == HCI_SUCCESS ==> BltEntry(new_s, vd, lba).lbamode == RAM)
  && (result == HCI_SUCCESS ==> BltEntry(new_s, vd, lba).output_addr == data)
  && (result == HCI_SUCCESS ==> BltEntry(new_s, vd, lba).memattr == MEMATTR_CACHEABLE)
  && (result == HCI_SUCCESS ==> BltEntry(new_s, vd, lba).shareability == SHAREABILITY_INNER)
  && ((!(src % EXTENT_SIZE == 0) &&
       ExtentAt(old_s, src).state == EXTENT_ACCESSIBLE &&
       !(data % EXTENT_SIZE == 0) &&
       ExtentAt(old_s, data).state == ENROLLED &&
       !(data >= 2^48 && !(VaultAt(old_s, vd).feat_bigaddr == FEATURE_TRUE)) &&
       !(vd % EXTENT_SIZE == 0) &&
       ExtentAt(old_s, vd).state == VD_STATE &&
       !(lba % EXTENT_SIZE == 0) &&
       VaultAt(old_s, vd).lba_protected.contains(lba) &&
       VaultAt(old_s, vd).state == VAULT_NEW &&
       !(RttWalk(old_s, vd, KEEPER_BLT_PAGE_LEVEL as int).state != UNASSIGNED))
    ==> result == HCI_SUCCESS)
  && (result != HCI_SUCCESS
    ==> ExtentAt(new_s, data).state == ExtentAt(old_s, data).state)
  && (result != HCI_SUCCESS
    ==> BltEntry(new_s, vd, lba).state == BltEntry(old_s, vd, lba).state)
  && (result != HCI_SUCCESS
    ==> BltEntry(new_s, vd, lba).lbamode == BltEntry(old_s, vd, lba).lbamode)
  && (result != HCI_SUCCESS
    ==> BltEntry(new_s, vd, lba).output_addr == BltEntry(old_s, vd, lba).output_addr)
  && (result != HCI_SUCCESS
    ==> BltEntry(new_s, vd, lba).memattr == BltEntry(old_s, vd, lba).memattr)
  && (result != HCI_SUCCESS
    ==> BltEntry(new_s, vd, lba).shareability == BltEntry(old_s, vd, lba).shareability)
  && (VaultAt(new_s, vd).ifp == VaultAt(old_s, vd).ifp)
}