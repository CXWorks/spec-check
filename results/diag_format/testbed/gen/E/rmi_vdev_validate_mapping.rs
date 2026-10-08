pub open spec fn rmi_vdev_validate_mapping_spec(rd: Address, rec_ptr: Address, pdev_ptr: Address, vdev_ptr: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RecAt(old_s, rec_ptr).state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
  && (RecAt(old_s, rec_ptr).owner != RealmAt(old_s, rd) ==> ResultEqual(result, RMI_ERROR_REC))
  && (!AddrIsGranuleAligned(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (VdevAt(old_s, vdev_ptr).pdev != pdev_ptr ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (top <= base ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (base != RecAt(old_s, rec_ptr).dev_mem_addr ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (top > RecAt(old_s, rec_ptr).dev_mem_top ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRttLevelAligned(old_s, base, RMM_RTT_PAGE_LEVEL as int) ==> ResultEqual(result, RMI_ERROR_RTT(0)))
  && (!AddrIsGranuleAligned(old_s, top) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (base == RttSkipEntriesWithRipas(old_s, RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)), RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)).level, base, top, false) ==> ResultEqual(result, RMI_ERROR_RTT(0)))
  && (RecAt(old_s, rec_ptr).dev_mem_flags.coh == DEV_MEM_NON_COHERENT && exists|rtte: RmmRttEntry| RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)).entries[rtte].attr_prot != MEMATTR_NON_CACHEABLE ==> ResultEqual(result, RMI_ERROR_RTT(0)))
  && (RecAt(old_s, rec_ptr).dev_mem_flags.coh == DEV_MEM_NON_COHERENT && exists|rtte: RmmRttEntry| RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)).entries[rtte].sh != SHAREABILITY_OUTER ==> ResultEqual(result, RMI_ERROR_RTT(0)))
  && (RecAt(old_s, rec_ptr).dev_mem_flags.coh == DEV_MEM_COHERENT && exists|rtte: RmmRttEntry| RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)).entries[rtte].attr_prot != MEMATTR_PASSTHROUGH ==> ResultEqual(result, RMI_ERROR_RTT(0)))
  && (RecAt(old_s, rec_ptr).dev_mem_flags.coh == DEV_MEM_COHERENT && exists|rtte: RmmRttEntry| RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)).entries[rtte].sh == SHAREABILITY_INNER ==> ResultEqual(result, RMI_ERROR_RTT(0)))
  && (!RttSkipEntriesWithRipas(old_s, RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)), RttWalk_(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level, RecAt(old_s, rec_ptr).dev_mem_pa, RttSkipEntriesWithRipas(old_s, RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)), RttWalk_(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level, base, top, false), false) ==> ResultEqual(result, RMI_ERROR_RTT(0)))
  && (exists|addr: Address| AddrInRange(old_s, addr, base, top - base + 1) && RealmIsLive(old_s, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_RTT(0)))
  && (result.is_Ok() ==> RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)).entries[0].ripas == DEV)
  && (result.is_Ok() ==> RecAt(new_s, rec_ptr).dev_mem_addr == min(top, RttSkipEntriesWithRipas(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)), RttWalk_(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level, base, top, false)))
  && (result.is_Ok() ==> RecAt(new_s, rec_ptr).dev_mem_pa == RecAt(new_s, rec_ptr).dev_mem_pa + (RttSkipEntriesWithRipas(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)), RttWalk_(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level, base, top, false) - base))
  && (result.is_Ok() ==> out_top == min(top, RttSkipEntriesWithRipas(new_s, RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)), RttWalk_(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level, base, top, false)))
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       AddrIsGranuleAligned(old_s, rec_ptr) &&
       PaIsDelegable(old_s, rec_ptr) &&
       !(GranuleAt(old_s, rec_ptr).state != REC) &&
       !(RecAt(old_s, rec_ptr).state == REC_RUNNING) &&
       !(RecAt(old_s, rec_ptr).owner != RealmAt(old_s, rd)) &&
       AddrIsGranuleAligned(old_s, pdev_ptr) &&
       PaIsDelegable(old_s, pdev_ptr) &&
       !(GranuleAt(old_s, pdev_ptr).state != PDEV) &&
       AddrIsGranuleAligned(old_s, vdev_ptr) &&
       PaIsDelegable(old_s, vdev_ptr) &&
       !(GranuleAt(old_s, vdev_ptr).state != VDEV) &&
       !(VdevAt(old_s, vdev_ptr).pdev != pdev_ptr) &&
       !(top <= base) &&
       !(base != RecAt(old_s, rec_ptr).dev_mem_addr) &&
       !(top > RecAt(old_s, rec_ptr).dev_mem_top) &&
       AddrIsRttLevelAligned(old_s, base, RMM_RTT_PAGE_LEVEL as int) &&
       AddrIsGranuleAligned(old_s, top) &&
       !(base == RttSkipEntriesWithRipas(old_s, RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)), RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)).level, base, top, false)) &&
       !(RecAt(old_s, rec_ptr).dev_mem_flags.coh == DEV_MEM_NON_COHERENT && exists|rtte: RmmRttEntry| RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)).entries[rtte].attr_prot != MEMATTR_NON_CACHEABLE) &&
       !(RecAt(old_s, rec_ptr).dev_mem_flags.coh == DEV_MEM_NON_COHERENT && exists|rtte: RmmRttEntry| RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)).entries[rtte].sh != SHAREABILITY_OUTER) &&
       !(RecAt(old_s, rec_ptr).dev_mem_flags.coh == DEV_MEM_COHERENT && exists|rtte: RmmRttEntry| RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)).entries[rtte].attr_prot != MEMATTR_PASSTHROUGH) &&
       !(RecAt(old_s, rec_ptr).dev_mem_flags.coh == DEV_MEM_COHERENT && exists|rtte: RmmRttEntry| RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)).entries[rtte].sh == SHAREABILITY_INNER) &&
       RttSkipEntriesWithRipas(old_s, RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)), RttWalk_(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level, RecAt(old_s, rec_ptr).dev_mem_pa, RttSkipEntriesWithRipas(old_s, RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)), RttWalk_(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level, base, top, false), false) &&
       !(exists|addr: Address| AddrInRange(old_s, addr, base, top - base + 1) && RealmIsLive(old_s, RealmAt(old_s, rd))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)).entries[0].ripas == RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)).entries[0].ripas)
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).dev_mem_addr == RecAt(old_s, rec_ptr).dev_mem_addr)
  && (result.is_Err()
    ==> RecAt(new_s, rec_ptr).dev_mem_pa == RecAt(old_s, rec_ptr).dev_mem_pa)
  && (result.is_Err()
    ==> out_top == out_top)
  && (!(result.is_Ok() && (RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)).entries[0].ripas == DEV)) ==> RttAt(new_s, RttWalk_(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)).entries[0].ripas == RttAt(old_s, RttWalk_(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int)).entries[0].ripas)
}