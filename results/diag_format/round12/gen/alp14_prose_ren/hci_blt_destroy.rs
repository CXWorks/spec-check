pub open spec fn hci_blt_destroy_spec(vd: Address, lba: Address, level: Int64, result: Hcicommandreturncode, blt: Address, top: Address, old_s: S, new_s: S) -> bool {
  ((!(vd % extent_size(old_s)) ==> HCI_ERROR_INPUT) &&
   (!is_enrollable_physical_address(old_s, vd) ==> HCI_ERROR_INPUT) &&
   (!Extentat(old_s, vd).in_vd_state(old_s) ==> HCI_ERROR_INPUT) &&
   (!is_valid_blt_level(old_s, Vaultat(old_s, vd), level) ==> HCI_ERROR_INPUT) &&
   (!is_valid_blt_level(old_s, Vaultat(old_s, vd), Vaultat(old_s, vd).rtt_level_start) ==> HCI_ERROR_INPUT) &&
   (!lba % address_range_size(old_s, Bltentryat(new_s, Bltat(new_s, Bltat(new_s, Bltat(old_s, vd).blt_addr), 0 as int), 0 as int).level - 1 as int) ==> HCI_ERROR_INPUT) &&
   ((lba >= pow2(Vaultat(old_s, vd).lba_width as nat)) ==> HCI_ERROR_INPUT) &&
  (result == HCI_SUCCESS && Bltentryat(new_s, Bltat(new_s, Bltat(new_s, Bltat(old_s, vd).blt_addr), 0 as int), 0 as int).state == UNASSIGNED && is_protected_address(old_s, lba) ==> true) &&
  (result == HCI_SUCCESS && Bltentryat(new_s, Bltat(new_s, Bltat(new_s, Bltat(old_s, vd).blt_addr), 0 as int), 0 as int).LBAMODE == DESTROYED && is_protected_address(old_s, lba) ==> true) &&
  (result == HCI_SUCCESS && Bltentryat(new_s, Bltat(new_s, Bltat(new_s, Bltat(old_s, vd).blt_addr), 0 as int), 0 as int).state == UNASSIGNED_PUB && !is_protected_address(old_s, lba) ==> true) &&
  (result == HCI_SUCCESS && Extentat(new_s, Bltentryat(new_s, Bltat(new_s, Bltat(new_s, Bltat(old_s, vd).blt_addr), 0 as int), 0 as int).addr).state == ENROLLED ==> true) &&
  (result == HCI_SUCCESS ==> Bltentryat(new_s, Bltat(new_s, Bltat(new_s, Bltat(old_s, vd).blt_addr), 0 as int), 0 as int).addr == blt) &&
  (result == HCI_SUCCESS ==> Bltentryat(new_s, Bltat(new_s, Bltat(new_s, Bltat(old_s, vd).blt_addr), 0 as int), 0 as int).addr == top) &&
  ((result == HCI_ERROR_BLT && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.state == TABLE) ==> HCI_ERROR_BLT) &&
  ((result == HCI_ERROR_BLT && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.state != TABLE) ==> HCI_ERROR_BLT) &&
  ((result == HCI_ERROR_BLT && Bltentryat(new_s, Bltat(new_s, Bltat(new_s, Bltat(old_s, vd).blt_addr), 0 as int), 0 as int).state == LIVE) ==> HCI_ERROR_BLT) &&
  ((result == HCI_ERROR_BLT && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.state == AUXILIARY) ==> HCI_ERROR_BLT) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.state == UNASSIGNED && !is_protected_address(old_s, lba) ==> true) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.LBAMODE == DESTROYED && !is_protected_address(old_s, lba) ==> true) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.state == UNASSIGNED_PUB && is_protected_address(old_s, lba) ==> true) &&
  (result == HCI_SUCCESS && Extentat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr).state == ENROLLED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr == blt) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr == top) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.state == UNASSIGNED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.LBAMODE == DESTROYED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.state == UNASSIGNED_PUB && !is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Extentat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr).state == ENROLLED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr == blt) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr == top) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.state == UNASSIGNED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.LBAMODE == DESTROYED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.state == UNASSIGNED_PUB && !is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Extentat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr).state == ENROLLED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr == blt) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr == top) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.state == UNASSIGNED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.LBAMODE == DESTROYED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.state == UNASSIGNED_PUB && !is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Extentat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr).state == ENROLLED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr == blt) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr == top) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.state == UNASSIGNED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.LBAMODE == DESTROYED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.state == UNASSIGNED_PUB && !is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Extentat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr).state == ENROLLED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr == blt) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr == top) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.state == UNASSIGNED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.LBAMODE == DESTROYED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.state == UNASSIGNED_PUB && !is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Extentat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr).state == ENROLLED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr == blt) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr == top) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.state == UNASSIGNED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.LBAMODE == DESTROYED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.state == UNASSIGNED_PUB && !is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Extentat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr).state == ENROLLED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr == blt) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr == top) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.state == UNASSIGNED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.LBAMODE == DESTROYED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.state == UNASSIGNED_PUB && !is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Extentat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr).state == ENROLLED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr == blt) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr == top) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.state == UNASSIGNED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.LBAMODE == DESTROYED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.state == UNASSIGNED_PUB && !is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Extentat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr).state == ENROLLED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr == blt) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr == top) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.state == UNASSIGNED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.LBAMODE == DESTROYED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.state == UNASSIGNED_PUB && !is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Extentat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr).state == ENROLLED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr == blt) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr == top) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.state == UNASSIGNED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.LBAMODE == DESTROYED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.state == UNASSIGNED_PUB && !is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Extentat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr).state == ENROLLED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr == blt) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr == top) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.state == UNASSIGNED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.LBAMODE == DESTROYED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.state == UNASSIGNED_PUB && !is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Extentat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr).state == ENROLLED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr == blt) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr == top) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.state == UNASSIGNED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.LBAMODE == DESTROYED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.state == UNASSIGNED_PUB && !is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Extentat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr).state == ENROLLED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr == blt) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr == top) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.state == UNASSIGNED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.LBAMODE == DESTROYED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.state == UNASSIGNED_PUB && !is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Extentat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr).state == ENROLLED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr == blt) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr == top) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.state == UNASSIGNED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.LBAMODE == DESTROYED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.state == UNASSIGNED_PUB && !is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Extentat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr).state == ENROLLED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr == blt) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.addr == top) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.state == UNASSIGNED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.LBAMODE == DESTROYED && is_protected_address(old_s, lba)) &&
  (result == HCI_SUCCESS && Bltwalk(new_s, Vaultat(new_s, vd), lba,level - 1 as int,KEEPER_BLT_TREE_PRIMARY).blte.state == UNASSIGNED_PUB && !is_protected