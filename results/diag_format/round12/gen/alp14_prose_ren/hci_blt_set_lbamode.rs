pub open spec fn hci_blt_set_lbamode_spec(vd: Address, worker_ptr: Address, base: Address, top: Address, result: Hcicommandreturncode, out_top: Address, old_s: S, new_s: S) -> bool {
  (is_aligned_to(vd, size_of::<Extent>() as nat) ==> result == HCI_SUCCESS)
  && (is_enrollable_physical_address(vd) ==> result == HCI_SUCCESS)
  && (Extentat(old_s, vd).state == VD_STATE ==> result == HCI_SUCCESS)
  && (is_aligned_to(worker_ptr, size_of::<Extent>() as nat) ==> result == HCI_SUCCESS)
  && (is_enrollable_physical_address(worker_ptr) ==> result == HCI_SUCCESS)
  && (Extentat(old_s, worker_ptr).state == WORKER_STATE ==> result == HCI_SUCCESS)
  && (Workerat(old_s, worker_ptr).state == WORKER_RUNNING ==> HCI_ERROR_WORKER)
  && (Workerat(old_s, worker_ptr).vault != Vaultat(old_s, vd) ==> HCI_ERROR_WORKER)
  && (top <= base ==> result == HCI_ERROR_INPUT)
  && (base != Workerat(old_s, worker_ptr).lbamode_addr ==> result == HCI_ERROR_INPUT)
  && (top > Workerat(old_s, worker_ptr).lbamode_top ==> result == HCI_ERROR_INPUT)
  && (Bltwalk(old_s, Vaultat(old_s, vd), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blte.lbamode != Workerat(old_s, worker_ptr).lbamode_value && is_aligned_to(base, pow2(Keptreeentrysize(Bltwalk(old_s, Vaultat(old_s, vd), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level)) as nat) ==> HCI_ERROR_BLT(Bltwalk(new_s, Vaultat(new_s, vd), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level as int))
  && (is_aligned_to(top, size_of::<Extent>() as nat) ==> result == HCI_SUCCESS)
  && (Bltwalk(old_s, Vaultat(old_s, vd), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blte.top == base && Bltwalk(old_s, Vaultat(old_s, vd), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blte.lbamode != Workerat(old_s, worker_ptr).lbamode_value ==> HCI_ERROR_BLT(Bltwalk(new_s, Vaultat(new_s, vd), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level as int))
  && (any_part_of_range_live_in_auxiliary_blt(old_s, Vaultat(old_s, vd), base, top) ==> HCI_ERROR_BLT(Bltwalk(new_s, Vaultat(new_s, vd), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level as int))
  && (result == HCI_SUCCESS ==> Workerat(new_s, worker_ptr).lbamode_addr == min(top, Bltskipentrieswithlbamode(new_s, Bltat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr), Bltwalk(new_s, Vaultat(new_s, vd), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level, base, top, (Workerat(new_s, worker_ptr).lbamode_value == RAM) && (Workerat(new_s, worker_ptr).lbamode_destroyed != CHANGE_DESTROYED)).0))
  && (result == HCI_SUCCESS ==> out_top == min(top, Bltskipentrieswithlbamode(new_s, Bltat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr), Bltwalk(new_s, Vaultat(new_s, vd), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level, base, top, (Workerat(new_s, worker_ptr).lbamode_value == RAM) && (Workerat(new_s, worker_ptr).lbamode_destroyed != CHANGE_DESTROYED)).0))
  && ((!(is_aligned_to(vd, size_of::<Extent>() as nat)) &&
       !(is_enrollable_physical_address(vd)) &&
       !(Extentat(old_s, vd).state == VD_STATE) &&
       !(is_aligned_to(worker_ptr, size_of::<Extent>() as nat)) &&
       !(is_enrollable_physical_address(worker_ptr)) &&
       !(Extentat(old_s, worker_ptr).state == WORKER_STATE) &&
       !(Workerat(old_s, worker_ptr).state == WORKER_RUNNING) &&
       !(Workerat(old_s, worker_ptr).vault != Vaultat(old_s, vd)) &&
       !(top <= base) &&
       !(base != Workerat(old_s, worker_ptr).lbamode_addr) &&
       !(top > Workerat(old_s, worker_ptr).lbamode_top) &&
       !(is_aligned_to(top, size_of::<Extent>() as nat)))
    ==> result == HCI_SUCCESS)
  && (result != HCI_SUCCESS
    ==> Workerat(new_s, worker_ptr).lbamode_addr == Workerat(old_s, worker_ptr).lbamode_addr)
  && (result != HCI_SUCCESS
    ==> out_top == 0)
  && (Bltwalk(new_s, Vaultat(new_s, vd), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blte.lbamode == Bltwalk(old_s, Vaultat(old_s, vd), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blte.lbamode
    ==> result != HCI_SUCCESS)
}