pub open spec fn hci_data_create_unknown_spec(vd: Vd, data: Data, lba: Lba, result: Result<(), HciStatusCode>, old_s: S, new_s: S) -> bool {
  ((data) % EXTENT_SIZE != 0 ==> result == HCI_ERROR_INPUT)
  && (!is_enrollable_physical_address_in_dram(old_s, data) ==> result == HCI_ERROR_INPUT)
  && (ExtentAt(old_s, data).state != ENROLLED ==> result == HCI_ERROR_INPUT)
  && ((!(ImplFeatures(old_s).feat_bigaddr == FEATURE_TRUE) && data >= 2^48) ==> result == HCI_ERROR_INPUT)
  && ((vd) % EXTENT_SIZE != 0 ==> result == HCI_ERROR_INPUT)
  && (!is_enrollable_physical_address(old_s, vd) ==> result == HCI_ERROR_INPUT)
  && (ExtentAt(old_s, vd).state != VD_STATE ==> result == HCI_ERROR_INPUT)
  && ((lba) % EXTENT_SIZE != 0 ==> result == HCI_ERROR_INPUT)
  && (!is_within_protected_lba_space(old_s, lba) ==> result == HCI_ERROR_INPUT)
  && (result == HCI_ERROR_BLT(0) ==> BltEntryAt(new_s, lba, 0 as int).level > 0)
  && (result == HCI_ERROR_BLT(0) ==> BltEntryAt(new_s, lba, 0 as int).state != UNASSIGNED)
  && (result == HCI_SUCCESS ==> ExtentAt(new_s, data).state == DATA)
  && (result == HCI_SUCCESS ==> BltEntryAt(new_s, lba, 0 as int).state == ASSIGNED)
  && (result == HCI_SUCCESS ==> BltEntryAt(new_s, lba, 0 as int).output_address == data)
  && (result == HCI_SUCCESS ==> BltEntryAt(new_s, lba, 0 as int).memory_attribute == MEMATTR_CACHEABLE)
  && (result == HCI_SUCCESS ==> BltEntryAt(new_s, lba, 0 as int).shareability == SHAREABILITY_INNER)
  && ((!( (data) % EXTENT_SIZE != 0 &&
         !is_enrollable_physical_address_in_dram(old_s, data) &&
         ExtentAt(old_s, data).state != ENROLLED &&
         ((!(ImplFeatures(old_s).feat_bigaddr == FEATURE_TRUE) && data >= 2^48)) &&
         ((vd) % EXTENT_SIZE != 0) &&
         !is_enrollable_physical_address(old_s, vd) &&
         ExtentAt(old_s, vd).state != VD_STATE &&
         ((lba) % EXTENT_SIZE != 0) &&
         !is_within_protected_lba_space(old_s, lba))
    ==> result == HCI_SUCCESS)
  && (result != HCI_SUCCESS
    ==> ExtentAt(new_s, data).state == ExtentAt(old_s, data).state)
  && (result != HCI_SUCCESS
    ==> BltEntryAt(new_s, lba, 0 as int).state == BltEntryAt(old_s, lba, 0 as int).state)
  && (result != HCI_SUCCESS
    ==> BltEntryAt(new_s, lba, 0 as int).output_address == BltEntryAt(old_s, lba, 0 as int).output_address)
  && (result != HCI_SUCCESS
    ==> BltEntryAt(new_s, lba, 0 as int).memory_attribute == BltEntryAt(old_s, lba, 0 as int).memory_attribute)
  && (result != HCI_SUCCESS
    ==> BltEntryAt(new_s, lba, 0 as int).shareability == BltEntryAt(old_s, lba, 0 as int).shareability)
}