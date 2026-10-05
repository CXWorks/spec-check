pub open spec fn hci_data_create_spec(vd: Address, data: Address, lba: Address, src: Address, flags: Hcidataflags, result: Hcicommandreturncode, old_s: S, new_s: S) -> bool {
  (src % EXTENT_SIZE != 0 ==> result == HCI_ERROR_INPUT)
  && (Extentat(old_s, src).state != ENROLLED ==> result == HCI_ERROR_INPUT)
  && (data % EXTENT_SIZE != 0 ==> result == HCI_ERROR_INPUT)
  && (Extentat(old_s, data).state != ENROLLED ==> result == HCI_ERROR_INPUT)
  && ((data) >= 2^48 && Vaultat(old_s, vd).feat_bigaddr == FEATURE_FALSE ==> result == HCI_ERROR_INPUT)
  && (vd % EXTENT_SIZE != 0 ==> result == HCI_ERROR_INPUT)
  && (Extentat(old_s, vd).state != VD_STATE ==> result == HCI_ERROR_INPUT)
  && (lba % EXTENT_SIZE != 0 ==> result == HCI_ERROR_INPUT)
  && (!lba_in_protected_lba_space(old_s, Vaultat(old_s, vd), lba) ==> result == HCI_ERROR_INPUT)
  && (Vaultat(old_s, vd).state != VAULT_NEW ==> result == HCI_ERROR_VAULT)
  && (Bltwalk(old_s, Vaultat(old_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level != KEEPER_BLT_PAGE_LEVEL ==> result == HCI_ERROR_BLT(Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level as int))
  && (Bltentryat(new_s, Bltat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr), Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level)) != UNASSIGNED ==> result == HCI_ERROR_BLT(Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level as int))
  && (result == HCI_SUCCESS ==> Extentat(new_s, data).state == DATA)
  && (result == HCI_SUCCESS ==> Bltentryat(new_s, Bltat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr), Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level)) == ASSIGNED)
  && (result == HCI_SUCCESS ==> Bltentryat(new_s, Bltat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr), Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level)).lbamode == RAM)
  && (result == HCI_SUCCESS ==> Bltentryat(new_s, Bltat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr), Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level)).output_addr == data)
  && (result == HCI_SUCCESS ==> Bltentryat(new_s, Bltat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr), Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level)).memattr == MEMATTR_CACHEABLE)
  && (result == HCI_SUCCESS ==> Bltentryat(new_s, Bltat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr), Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level)).shareability == SHAREABILITY_INNER)
  && ((!(src % EXTENT_SIZE != 0) &&
       !(Extentat(old_s, src).state != ENROLLED) &&
       !(data % EXTENT_SIZE != 0) &&
       !(Extentat(old_s, data).state != ENROLLED) &&
       !((data) >= 2^48 && Vaultat(old_s, vd).feat_bigaddr == FEATURE_FALSE) &&
       !(vd % EXTENT_SIZE != 0) &&
       !(Extentat(old_s, vd).state != VD_STATE) &&
       !(lba % EXTENT_SIZE != 0) &&
       lba_in_protected_lba_space(old_s, Vaultat(old_s, vd), lba) &&
       !(Vaultat(old_s, vd).state != VAULT_NEW) &&
       !(Bltwalk(old_s, Vaultat(old_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level != KEEPER_BLT_PAGE_LEVEL) &&
       !(Bltentryat(old_s, Bltat(old_s, Bltwalk(old_s, Vaultat(old_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr), Bltentryindex(old_s, lba, Bltwalk(old_s, Vaultat(old_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level)) != UNASSIGNED))
    ==> result == HCI_SUCCESS)
  && (result != HCI_SUCCESS
    ==> Extentat(new_s, data).state == Extentat(old_s, data).state)
  && (result != HCI_SUCCESS
    ==> Bltentryat(new_s, Bltat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr), Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level)) == Bltentryat(old_s, Bltat(old_s, Bltwalk(old_s, Vaultat(old_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr), Bltentryindex(old_s, lba, Bltwalk(old_s, Vaultat(old_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level)))
  && (result != HCI_SUCCESS
    ==> Bltentryat(new_s, Bltat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr), Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level)).lbamode == Bltentryat(old_s, Bltat(old_s, Bltwalk(old_s, Vaultat(old_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr), Bltentryindex(old_s, lba, Bltwalk(old_s, Vaultat(old_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level)).lbamode)
  && (result != HCI_SUCCESS
    ==> Bltentryat(new_s, Bltat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr), Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level)).output_addr == Bltentryat(old_s, Bltat(old_s, Bltwalk(old_s, Vaultat(old_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr), Bltentryindex(old_s, lba, Bltwalk(old_s, Vaultat(old_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level)).output_addr)
  && (result != HCI_SUCCESS
    ==> Bltentryat(new_s, Bltat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr), Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level)).memattr == Bltentryat(old_s, Bltat(old_s, Bltwalk(old_s, Vaultat(old_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr), Bltentryindex(old_s, lba, Bltwalk(old_s, Vaultat(old_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level)).memattr)
  && (result != HCI_SUCCESS
    ==> Bltentryat(new_s, Bltat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr), Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level)).shareability == Bltentryat(old_s, Bltat(old_s, Bltwalk(old_s, Vaultat(old_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr), Bltentryindex(old_s, lba, Bltwalk(old_s, Vaultat(old_s, vd), lba, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level)).shareability)
  && (Vaultat(new_s, vd).state == Vaultat(old_s, vd).state)
}