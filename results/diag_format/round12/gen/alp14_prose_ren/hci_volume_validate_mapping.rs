pub open spec fn hci_volume_validate_mapping_spec(vd: Address, worker_ptr: Address, drive_ptr: Address, volume_ptr: Address, base: Address, top: Address, result: Hcicommandreturncode, out_top: Address, old_s: S, new_s: S) -> bool {
  ((!(Vaultat(old_s, vd).is_aligned_to(Extent::size(old_s))) ==> result == HCI_ERROR_INPUT)
   && (!(Vaultat(old_s, vd).is_enrollable_physical_address(old_s)) ==> result == HCI_ERROR_INPUT)
   && (!(Extentat(old_s, vd).is_in_VD_state(old_s)) ==> result == HCI_ERROR_INPUT)
   && (!(Workerat(old_s, worker_ptr).is_aligned_to(Extent::size(old_s))) ==> result == HCI_ERROR_INPUT)
   && (!(Workerat(old_s, worker_ptr).is_enrollable_physical_address(old_s)) ==> result == HCI_ERROR_INPUT)
   && (!(Extentat(old_s, worker_ptr).is_in_WORKER_state(old_s)) ==> result == HCI_ERROR_INPUT)
   && (Workerat(old_s, worker_ptr).state == WORKER_RUNNING ==> result == HCI_ERROR_WORKER)
   && (Workerat(old_s, worker_ptr).owner != Vaultat(old_s, vd).id ==> result == HCI_ERROR_WORKER)
   && (!(Driveat(old_s, drive_ptr).is_aligned_to(Extent::size(old_s))) ==> result == HCI_ERROR_INPUT)
   && (!(Driveat(old_s, drive_ptr).is_enrollable_physical_address(old_s)) ==> result == HCI_ERROR_INPUT)
   && (!(Extentat(old_s, drive_ptr).is_in_DRIVE_state(old_s)) ==> result == HCI_ERROR_INPUT)
   && (!(Volumeat(old_s, volume_ptr).is_aligned_to(Extent::size(old_s))) ==> result == HCI_ERROR_INPUT)
   && (!(Volumeat(old_s, volume_ptr).is_enrollable_physical_address(old_s)) ==> result == HCI_ERROR_INPUT)
   && (!(Extentat(old_s, volume_ptr).is_in_VOLUME_state(old_s)) ==> result == HCI_ERROR_INPUT)
   && (Volumeat(old_s, volume_ptr).drive != drive_ptr ==> result == HCI_ERROR_DEVICE)
   && (top <= base ==> result == HCI_ERROR_INPUT)
   && (Workerat(old_s, worker_ptr).dev_mem_addr != base ==> result == HCI_ERROR_INPUT)
   && (top > Workerat(old_s, worker_ptr).dev_mem_top ==> result == HCI_ERROR_INPUT)
   && (base % Bltentrysizeat(old_s, Bltwalk(old_s, Vaultat(old_s, vd), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int)) != 0 ==> result == HCI_ERROR_BLT)
   && (top % Extent::size(old_s) != 0 ==> result == HCI_ERROR_INPUT)
   && (base == Bltskipentrieswithlbamode(old_s, Bltat(old_s, Bltwalk(old_s, Vaultat(old_s, vd), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr), Bltwalk(old_s, Vaultat(old_s, vd), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level, base, top, false).0 ==> result == HCI_ERROR_BLT)
   && (Workerat(old_s, worker_ptr).dev_mem_non_coherent && exists(s: S, e: Bltentry(s, Bltwalk(new_s, Vaultat(new_s, vd), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr)) ==> result == HCI_ERROR_BLT)
   && (Workerat(old_s, worker_ptr).dev_mem_non_coherent && exists(s: S, e: Bltentry(s, Bltwalk(new_s, Vaultat(new_s, vd), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr)) ==> result == HCI_ERROR_BLT)
   && (Workerat(old_s, worker_ptr).dev_mem_coherent && exists(s: S, e: Bltentry(s, Bltwalk(new_s, Vaultat(new_s, vd), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr)) ==> result == HCI_ERROR_BLT)
   && (Workerat(old_s, worker_ptr).dev_mem_coherent && exists(s: S, e: Bltentry(s, Bltwalk(new_s, Vaultat(new_s, vd), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr)) ==> result == HCI_ERROR_BLT)
   && (Bltat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr).output_address != Bltat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr).output_address + 1 ==> result == HCI_ERROR_BLT)
   && (exists(addr: Address, addr >= base && addr < top) ==> result == HCI_ERROR_BLT)
   && (result == HCI_SUCCESS ==> (forall (i: int), Bltat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr + i as int).lbamode == DEV))
   && (result == HCI_SUCCESS ==> Workerat(new_s, worker_ptr).dev_mem_addr == min(top, Bltskipentrieswithlbamode(new_s, Bltat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr), Bltwalk(new_s, Vaultat(new_s, vd), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level, base, top, false).0))
   && (result == HCI_SUCCESS ==> Workerat(new_s, worker_ptr).dev_mem_dpa == Workerat(old_s, worker_ptr).dev_mem_dpa + (Bltskipentrieswithlbamode(new_s, Bltat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr), Bltwalk(new_s, Vaultat(new_s, vd), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level, base, top, false).0 - base))
   && (result == HCI_SUCCESS ==> out_top == min(top, Bltskipentrieswithlbamode(new_s, Bltat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr), Bltwalk(new_s, Vaultat(new_s, vd), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level, base, top, false).0))
   && ((!(Vaultat(old_s, vd).is_aligned_to(Extent::size(old_s))) &&
        !(Vaultat(old_s, vd).is_enrollable_physical_address(old_s)) &&
        !(Extentat(old_s, vd).is_in_VD_state(old_s)) &&
        !(Workerat(old_s, worker_ptr).is_aligned_to(Extent::size(old_s))) &&
        !(Workerat(old_s, worker_ptr).is_enrollable_physical_address(old_s)) &&
        !(Extentat(old_s, worker_ptr).is_in_WORKER_state(old_s)) &&
        !(Workerat(old_s, worker_ptr).state == WORKER_RUNNING) &&
        !(Workerat(old_s, worker_ptr).owner != Vaultat(old_s, vd).id) &&
        !(Driveat(old_s, drive_ptr).is_aligned_to(Extent::size(old_s))) &&
        !(Driveat(old_s, drive_ptr).is_enrollable_physical_address(old_s)) &&
        !(Extentat(old_s, drive_ptr).is_in_DRIVE_state(old_s)) &&
        !(Volumeat(old_s, volume_ptr).is_aligned_to(Extent::size(old_s))) &&
        !(Volumeat(old_s, volume_ptr).is_enrollable_physical_address(old_s)) &&
        !(Extentat(old_s, volume_ptr).is_in_VOLUME_state(old_s)) &&
        !(Volumeat(old_s, volume_ptr).drive != drive_ptr) &&
        !(top <= base) &&
        !(Workerat(old_s, worker_ptr).dev_mem_addr != base) &&
        !(top > Workerat(old_s, worker_ptr).dev_mem_top) &&
        !(base % Bltentrysizeat(old_s, Bltwalk(old_s, Vaultat(old_s, vd), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int)) != 0) &&
        !(top % Extent::size(old_s) != 0) &&
        !(base == Bltskipentrieswithlbamode(old_s, Bltat(old_s, Bltwalk(old_s, Vaultat(old_s, vd), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr), Bltwalk(old_s, Vaultat(old_s, vd), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).level, base, top, false).0) &&
        !(Workerat(old_s, worker_ptr).dev_mem_non_coherent && exists(s: S, e: Bltentry(s, Bltwalk(new_s, Vaultat(new_s, vd), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr))) &&
        !(Workerat(old_s, worker_ptr).dev_mem_non_coherent && exists(s: S, e: Bltentry(s, Bltwalk(new_s, Vaultat(new_s, vd), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr))) &&
        !(Workerat(old_s, worker_ptr).dev_mem_coherent && exists(s: S, e: Bltentry(s, Bltwalk(new_s, Vaultat(new_s, vd), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr))) &&
        !(Workerat(old_s, worker_ptr).dev_mem_coherent && exists(s: S, e: Bltentry(s, Bltwalk(new_s, Vaultat(new_s, vd), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr))) &&
        !(Bltat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr).output_address != Bltat(new_s, Bltwalk(new_s, Vaultat(new_s, vd), base, KEEPER_BLT_PAGE_LEVEL as int, KEEPER_BLT_TREE_PRIMARY as int).blt_addr).output_address + 1) &&
        !(exists(addr: Address, addr >= base && addr < top)))
     ==> result == HCI_SUCCESS)
   && (result != HCI_SUCCESS
     ==> Workerat(new_s, worker_ptr).dev_mem_addr == Workerat(old_s, worker_ptr).dev_mem_addr)
   && (result != HCI_SUCCESS
     ==> Workerat(new_s, worker_ptr).dev_mem_dpa == Workerat(old_s, worker_ptr).dev_mem_dpa)
   && (result != HCI_SUCCESS
     ==> out_top == 0)
   && (!(result == HCI_SUCCESS && (Workerat(new_s, worker_ptr).dev_mem_addr == Workerat(old_s, worker_ptr).dev_mem_addr)) ==> result != HCI_SUCCESS)
}