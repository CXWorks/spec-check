pub open spec fn hci_vxlator_map_spec(vd: Address, vxlator_ptr: Address, lba: Address, level: Int64, result: Hcicommandreturncode, old_s: S, new_s: S) -> bool {
  (result == HCI_ERROR_NOT_SUPPORTED ==> true)
  && ((!(vd % extent_size(old_s) == 0) ==> result == HCI_ERROR_INPUT)
    ==> result == HCI_ERROR_INPUT)
  && (Vaultat(old_s, vd).state != VAULT_NEW ==> result == HCI_ERROR_INPUT)
  && (Vxlatorat(old_s, vxlator_ptr).state != VXLATOR ==> result == HCI_ERROR_INPUT)
  && (Bltat(old_s, Bltwalk(old_s, Vaultat(old_s, vd), lba, level as int, KEEPER_BLT_TREE_PRIMARY).blt_addr).entries[Bltentryindex(old_s, lba,Bltwalk(old_s, Vaultat(old_s, vd), lba, level as int, KEEPER_BLT_TREE_PRIMARY).level as int)].state != UNASSIGNED ==> result == HCI_ERROR_BLT(0))
  && (Bltat(old_s, Bltwalk(old_s, Vaultat(old_s, vd), lba, level as int, KEEPER_BLT_TREE_PRIMARY).blt_addr).entries[Bltentryindex(old_s, lba,Bltwalk(old_s, Vaultat(old_s, vd), lba, level as int, KEEPER_BLT_TREE_PRIMARY).level as int)].LBAMODE != EMPTY ==> result == HCI_ERROR_BLT(0))
  && (result == HCI_SUCCESS ==> Bltat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba, level as int, KEEPER_BLT_TREE_PRIMARY).blt_addr).entries[Bltentryindex(new_s, lba,Bltwalk(new_s, Vaultat(new_s, vd), lba, level as int, KEEPER_BLT_TREE_PRIMARY).level as int)].state == ASSIGNED_VXLATOR)
  && (result == HCI_SUCCESS ==> Bltat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba, level as int, KEEPER_BLT_TREE_PRIMARY).blt_addr).entries[Bltentryindex(new_s, lba,Bltwalk(new_s, Vaultat(new_s, vd), lba, level as int, KEEPER_BLT_TREE_PRIMARY).level as int)].addr == vxlator_ptr)
  && ((result != HCI_SUCCESS &&
       (result != HCI_ERROR_NOT_SUPPORTED &&
        (result == HCI_ERROR_INPUT || result == HCI_ERROR_BLT(0))))
    ==> Bltat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba, level as int, KEEPER_BLT_TREE_PRIMARY).blt_addr).entries[Bltentryindex(new_s, lba,Bltwalk(new_s, Vaultat(new_s, vd), lba, level as int, KEEPER_BLT_TREE_PRIMARY).level as int)].state == Bltat(old_s, Bltwalk(old_s, Vaultat(old_s, vd), lba, level as int, KEEPER_BLT_TREE_PRIMARY).blt_addr).entries[Bltentryindex(old_s, lba,Bltwalk(old_s, Vaultat(old_s, vd), lba, level as int, KEEPER_BLT_TREE_PRIMARY).level as int)].state)
  && ((result != HCI_SUCCESS &&
       (result != HCI_ERROR_NOT_SUPPORTED &&
        (result == HCI_ERROR_INPUT || result == HCI_ERROR_BLT(0))))
    ==> Bltat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba, level as int, KEEPER_BLT_TREE_PRIMARY).blt_addr).entries[Bltentryindex(new_s, lba,Bltwalk(new_s, Vaultat(new_s, vd), lba, level as int, KEEPER_BLT_TREE_PRIMARY).level as int)].addr == Bltat(old_s, Bltwalk(old_s, Vaultat(old_s, vd), lba, level as int, KEEPER_BLT_TREE_PRIMARY).blt_addr).entries[Bltentryindex(old_s, lba,Bltwalk(old_s, Vaultat(old_s, vd), lba, level as int, KEEPER_BLT_TREE_PRIMARY).level as int)].addr)
  && (result == HCI_SUCCESS
    ==> Bltat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba, level as int, KEEPER_BLT_TREE_PRIMARY).blt_addr).entries[Bltentryindex(new_s, lba,Bltwalk(new_s, Vaultat(new_s, vd), lba, level as int, KEEPER_BLT_TREE_PRIMARY).level as int)].vxlator_ptr == Bltat(old_s, Bltwalk(old_s, Vaultat(old_s, vd), lba, level as int, KEEPER_BLT_TREE_PRIMARY).blt_addr).entries[Bltentryindex(old_s, lba,Bltwalk(old_s, Vaultat(old_s, vd), lba, level as int, KEEPER_BLT_TREE_PRIMARY).level as int)].vxlator_ptr)
}