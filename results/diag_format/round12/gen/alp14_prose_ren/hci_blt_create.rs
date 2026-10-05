pub open spec fn hci_blt_create_spec(vd: Address, blt: Address, lba: Address, level: Int64, result: Hcicommandreturncode, old_s: S, new_s: S) -> bool {
  ((!(Vaultat(old_s, vd).is_aligned_to(Extent::size())) ==> result == HCI_ERROR_INPUT)
   && (!(is_enrollable_physical_address(old_s, vd) ==> result == HCI_ERROR_INPUT)
     ==> result == HCI_ERROR_INPUT)
   && (!(Extentat(old_s, vd).state == VD_STATE ==> result == HCI_ERROR_INPUT)
     ==> result == HCI_ERROR_INPUT)
   && (!(is_valid_blt_level_for_vault(old_s, vd, level) && !(level == Vaultat(old_s, vd).blt_level_start) ==> result == HCI_ERROR_INPUT)
     ==> result == HCI_ERROR_INPUT)
   && (!(lba.is_aligned_to(Bltentryat(new_s, Bltat(new_s, Bltat(new_s, Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1, KEEPER_BLT_TREE_PRIMARY).level as int).blt_addr), Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1, KEEPER_BLT_TREE_PRIMARY).level as int)).extent_size)) ==> result == HCI_ERROR_INPUT)
   && (!(lba >= pow2(Vaultat(old_s, vd).lba_width) ==> result == HCI_ERROR_INPUT)
     ==> result == HCI_ERROR_INPUT)
   && (!(Extentat(old_s, blt).state == ENROLLED ==> result == HCI_ERROR_INPUT)
     ==> result == HCI_ERROR_INPUT)
   && (!(Extentat(old_s, blt).state == ENROLLED ==> result == HCI_ERROR_INPUT)
     ==> result == HCI_ERROR_INPUT)
   && (result == HCI_ERROR_BLT ==> result == HCI_ERROR_BLT)
   && (result == HCI_ERROR_BLT ==> result == HCI_ERROR_BLT)
   && (result == HCI_SUCCESS ==> Extentat(new_s, blt).state == BLT_STATE)
   && (result == HCI_SUCCESS ==> Bltentryat(new_s, Bltat(new_s, Bltat(new_s, Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1, KEEPER_BLT_TREE_PRIMARY).level as int).blt_addr), Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1, KEEPER_BLT_TREE_PRIMARY).level as int)).state == TABLE_STATE)
   && (result == HCI_SUCCESS ==> Bltentryat(new_s, Bltat(new_s, Bltat(new_s, Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1, KEEPER_BLT_TREE_PRIMARY).level as int).blt_addr), Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1, KEEPER_BLT_TREE_PRIMARY).level as int)).blt == blt)
   && ((result == HCI_SUCCESS && is_protected_lba(old_s, vd, lba))
     ==> forall (i: int), Bltentryat(new_s, Bltat(new_s, Bltat(new_s, Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1, KEEPER_BLT_TREE_PRIMARY).level as int).blt_addr), Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1, KEEPER_BLT_TREE_PRIMARY).level as int)).lbamode[i] == Bltentryat(old_s, Bltat(old_s, Bltat(old_s, Bltentryindex(old_s, lba, Bltwalk(old_s, Vaultat(old_s, vd), lba,level - 1, KEEPER_BLT_TREE_PRIMARY).level as int).blt_addr), Bltentryindex(old_s, lba, Bltwalk(old_s, Vaultat(old_s, vd), lba,level - 1, KEEPER_BLT_TREE_PRIMARY).level as int)).lbamode[i])
   && (result == HCI_SUCCESS
     ==> forall (i: int), Bltentryat(new_s, Bltat(new_s, Bltat(new_s, Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1, KEEPER_BLT_TREE_PRIMARY).level as int).blt_addr), Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1, KEEPER_BLT_TREE_PRIMARY).level as int)).state == Bltentryat(old_s, Bltat(old_s, Bltat(old_s, Bltentryindex(old_s, lba, Bltwalk(old_s, Vaultat(old_s, vd), lba,level - 1, KEEPER_BLT_TREE_PRIMARY).level as int).blt_addr), Bltentryindex(old_s, lba, Bltwalk(old_s, Vaultat(old_s, vd), lba,level - 1, KEEPER_BLT_TREE_PRIMARY).level as int)).state)
   && ((result == HCI_SUCCESS && (Bltentryat(old_s, Bltat(old_s, Bltat(old_s, Bltentryindex(old_s, lba, Bltwalk(old_s, Vaultat(old_s, vd), lba,level - 1, KEEPER_BLT_TREE_PRIMARY).level as int).blt_addr), Bltentryindex(old_s, lba, Bltwalk(old_s, Vaultat(old_s, vd), lba,level - 1, KEEPER_BLT_TREE_PRIMARY).level as int)).state != UNASSIGNED || Bltentryat(old_s, Bltat(old_s, Bltat(old_s, Bltentryindex(old_s, lba, Bltwalk(old_s, Vaultat(old_s, vd), lba,level - 1, KEEPER_BLT_TREE_PRIMARY).level as int).blt_addr), Bltentryindex(old_s, lba, Bltwalk(old_s, Vaultat(old_s, vd), lba,level - 1, KEEPER_BLT_TREE_PRIMARY).level as int)).state != UNASSIGNED_PUB))
     ==> Bltentryat(new_s, Bltat(new_s, Bltat(new_s, Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1, KEEPER_BLT_TREE_PRIMARY).level as int).blt_addr), Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1, KEEPER_BLT_TREE_PRIMARY).level as int)).start_addr == Bltentryat(old_s, Bltat(old_s, Bltat(old_s, Bltentryindex(old_s, lba, Bltwalk(old_s, Vaultat(old_s, vd), lba,level - 1, KEEPER_BLT_TREE_PRIMARY).level as int).blt_addr), Bltentryindex(old_s, lba, Bltwalk(old_s, Vaultat(old_s, vd), lba,level - 1, KEEPER_BLT_TREE_PRIMARY).level as int)).start_addr)
   && ((!(!(Vaultat(old_s, vd).is_aligned_to(Extent::size())) &&
          is_enrollable_physical_address(old_s, vd) &&
          !(Extentat(old_s, vd).state == VD_STATE) &&
          (is_valid_blt_level_for_vault(old_s, vd, level) && !(level == Vaultat(old_s, vd).blt_level_start)) &&
          !(lba.is_aligned_to(Bltentryat(new_s, Bltat(new_s, Bltat(new_s, Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1, KEEPER_BLT_TREE_PRIMARY).level as int).blt_addr), Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1, KEEPER_BLT_TREE_PRIMARY).level as int)).extent_size)) &&
          !(lba >= pow2(Vaultat(old_s, vd).lba_width)) &&
          !(Extentat(old_s, blt).state == ENROLLED) &&
          !(Extentat(old_s, blt).state == ENROLLED))
     ==> result == HCI_SUCCESS)
   && (result != HCI_SUCCESS
     ==> Extentat(new_s, blt).state == Extentat(old_s, blt).state)
   && (result != HCI_SUCCESS
     ==> Bltentryat(new_s, Bltat(new_s, Bltat(new_s, Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1, KEEPER_BLT_TREE_PRIMARY).level as int).blt_addr), Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1, KEEPER_BLT_TREE_PRIMARY).level as int)).state == Bltentryat(old_s, Bltat(old_s, Bltat(old_s, Bltentryindex(old_s, lba, Bltwalk(old_s, Vaultat(old_s, vd), lba,level - 1, KEEPER_BLT_TREE_PRIMARY).level as int).blt_addr), Bltentryindex(old_s, lba, Bltwalk(old_s, Vaultat(old_s, vd), lba,level - 1, KEEPER_BLT_TREE_PRIMARY).level as int)).state)
   && (result != HCI_SUCCESS
     ==> Bltentryat(new_s, Bltat(new_s, Bltat(new_s, Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1, KEEPER_BLT_TREE_PRIMARY).level as int).blt_addr), Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1, KEEPER_BLT_TREE_PRIMARY).level as int)).blt == Bltentryat(old_s, Bltat(old_s, Bltat(old_s, Bltentryindex(old_s, lba, Bltwalk(old_s, Vaultat(old_s, vd), lba,level - 1, KEEPER_BLT_TREE_PRIMARY).level as int).blt_addr), Bltentryindex(old_s, lba, Bltwalk(old_s, Vaultat(old_s, vd), lba,level - 1, KEEPER_BLT_TREE_PRIMARY).level as int)).blt)
   && (result != HCI_SUCCESS
     ==> Bltentryat(new_s, Bltat(new_s, Bltat(new_s, Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1, KEEPER_BLT_TREE_PRIMARY).level as int).blt_addr), Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1, KEEPER_BLT_TREE_PRIMARY).level as int)).start_addr == Bltentryat(old_s, Bltat(old_s, Bltat(old_s, Bltentryindex(old_s, lba, Bltwalk(old_s, Vaultat(old_s, vd), lba,level - 1, KEEPER_BLT_TREE_PRIMARY).level as int).blt_addr), Bltentryindex(old_s, lba, Bltwalk(old_s, Vaultat(old_s, vd), lba,level - 1, KEEPER_BLT_TREE_PRIMARY).level as int)).start_addr)
   && (Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1, KEEPER_BLT_TREE_PRIMARY) == Bltwalk(old_s, Vaultat(old_s, vd), lba,level - 1, KEEPER_BLT_TREE_PRIMARY))
   && (Bltentryindex(new_s, lba, Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1, KEEPER_BLT_TREE_PRIMARY).level as int) == Bltentryindex(old_s, lba, Bltwalk(old_s, Vaultat(old_s, vd), lba,level - 1, KEEPER_BLT_TREE_PRIMARY).level as int))
}