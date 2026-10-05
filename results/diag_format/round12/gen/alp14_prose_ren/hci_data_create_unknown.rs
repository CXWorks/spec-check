pub open spec fn hci_data_create_unknown_spec(vd: Address, data: Address, lba: Address, result: Hcicommandreturncode, old_s: S, new_s: S) -> bool {
  ((data % extent_size(old_s) != 0) ==> result == HCI_ERROR_INPUT)
  && ((!(is_enrollable_physical_address_in_dram(old_s, data)) ) ==> result == HCI_ERROR_INPUT)
  && ((Extentat(old_s, data).state != ENROLLED) ==> result == HCI_ERROR_INPUT)
  && ((!(Vaultat(old_s, vd).feat_bigaddr == FEATURE_TRUE) && data >= (1 << 48)) ==> result == HCI_ERROR_INPUT)
  && ((vd % extent_size(old_s) != 0) ==> result == HCI_ERROR_INPUT)
  && ((!(is_enrollable_physical_address(old_s, vd)) ) ==> result == HCI_ERROR_INPUT)
  && ((Extentat(old_s, vd).state != VD_STATE) ==> result == HCI_ERROR_INPUT)
  && ((lba % extent_size(old_s) != 0) ==> result == HCI_ERROR_INPUT)
  && ((!(lba within_protected_lba_space(old_s, Vaultat(old_s, vd))) ) ==> result == HCI_ERROR_INPUT)
  && (result == HCI_ERROR_BLT(0) ==> Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level > KEEPER_BLT_PAGE_LEVEL)
  && (result == HCI_ERROR_BLT(0) ==> Bltentryat(new_s, Bltat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr), Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level as int).state != UNASSIGNED)
  && (result == HCI_SUCCESS ==> Extentat(new_s, data).state == DATA)
  && (result == HCI_SUCCESS ==> Bltentryat(new_s, Bltat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr), Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level as int)).state == ASSIGNED)
  && (result == HCI_SUCCESS ==> Bltentryat(new_s, Bltat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr), Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level as int)).output_addr == data)
  && (result == HCI_SUCCESS ==> Bltentryat(new_s, Bltat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr), Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level as int)).mem_attr == MEMATTR_CACHEABLE)
  && (result == HCI_SUCCESS ==> Bltentryat(new_s, Bltat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr), Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level as int)).shareability == SHAREABILITY_INNER)
  && ((!(result == HCI_ERROR_INPUT) &&
       result != HCI_ERROR_INPUT &&
       result != HCI_ERROR_INPUT &&
       result != HCI_ERROR_INPUT &&
       result != HCI_ERROR_INPUT &&
       result != HCI_ERROR_INPUT &&
       result != HCI_ERROR_INPUT &&
       result != HCI_ERROR_INPUT &&
       result != HCI_ERROR_INPUT &&
       result != HCI_ERROR_INPUT &&
       result != HCI_ERROR_BLT(0) &&
       result != HCI_ERROR_BLT(0))
    ==> Extentat(new_s, data).state == ENROLLED)
  && (result != HCI_SUCCESS
    ==> Extentat(new_s, data).state == Extentat(old_s, data).state)
  && (result != HCI_SUCCESS
    ==> Bltentryat(new_s, Bltat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr), Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level as int)).state == Bltentryat(old_s, Bltat(old_s, Bltwalk(old_s, Vaultat(old_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr), Bltentryindex(old_s, lba, Bltwalk(old_s, Vaultat(old_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level as int)).state)
  && (result != HCI_SUCCESS
    ==> Bltentryat(new_s, Bltat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr), Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level as int)).output_addr == Bltentryat(old_s, Bltat(old_s, Bltwalk(old_s, Vaultat(old_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr), Bltentryindex(old_s, lba, Bltwalk(old_s, Vaultat(old_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level as int)).output_addr)
  && (result != HCI_SUCCESS
    ==> Bltentryat(new_s, Bltat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr), Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level as int)).mem_attr == Bltentryat(old_s, Bltat(old_s, Bltwalk(old_s, Vaultat(old_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr), Bltentryindex(old_s, lba, Bltwalk(old_s, Vaultat(old_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level as int)).mem_attr)
  && (result != HCI_SUCCESS
    ==> Bltentryat(new_s, Bltat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr), Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level as int)).shareability == Bltentryat(old_s, Bltat(old_s, Bltwalk(old_s, Vaultat(old_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr), Bltentryindex(old_s, lba, Bltwalk(old_s, Vaultat(old_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level as int)).shareability)
  && (Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level == Bltwalk(old_s, Vaultat(old_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level)
  && (Bltentryat(new_s, Bltat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr), Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level as int)).state == Bltentryat(old_s, Bltat(old_s, Bltwalk(old_s, Vaultat(old_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr), Bltentryindex(old_s, lba, Bltwalk(old_s, Vaultat(old_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level as int)).state)
}