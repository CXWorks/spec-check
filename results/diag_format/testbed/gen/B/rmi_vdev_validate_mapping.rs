pub open spec fn rmi_vdev_validate_mapping_spec(rd: Address, rec_ptr: Address, pdev_ptr: Address, vdev_ptr: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (RecAt(old_s, rec_ptr).state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
    && (RecAt(old_s, rec_ptr).owner != rd ==> ResultEqual(result, RMI_ERROR_REC))
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
    && let walk = RttWalk_(old_s, rd, base, RMM_RTT_PAGE_LEVEL as int) in
    && let walk_level = walk.level in
    && let walk_rtt_addr = walk.rtt_addr in
    && let walk_top_pre = RttSkipEntriesWithRipas(RttAt(old_s, walk_rtt_addr), walk_level, base, top, false) in
    && (!AddrIsRttLevelAligned(old_s, base, walk_level) ==> ResultEqual(result, RMI_ERROR_RTT(walk_level as int)))
    && (!AddrIsGranuleAligned(old_s, top) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (base == walk_top_pre ==> ResultEqual(result, RMI_ERROR_RTT(walk_level as int)))
    && (RecAt(old_s, rec_ptr).dev_mem_flags.coh == DEV_MEM_NON_COHERENT && !RttEntriesInRangeMemAttr(RttAt(old_s, walk_rtt_addr), walk_level, base, walk_top_pre, MEMATTR_NON_CACHEABLE) ==> ResultEqual(result, RMI_ERROR_RTT(walk_level as int)))
    && (RecAt(old_s, rec_ptr).dev_mem_flags.coh == DEV_MEM_NON_COHERENT && !RttEntriesInRangeNonCohDevMem(RttAt(old_s, walk_rtt_addr), walk_level, base, walk_top_pre) ==> ResultEqual(result, RMI_ERROR_RTT(walk_level as int)))
    && (RecAt(old_s, rec_ptr).dev_mem_flags.coh == DEV_MEM_COHERENT && !RttEntriesInRangeMemAttr(RttAt(old_s, walk_rtt_addr), walk_level, base, walk_top_pre, MEMATTR_PASSTHROUGH) ==> ResultEqual(result, RMI_ERROR_RTT(walk_level as int)))
    && (RecAt(old_s, rec_ptr).dev_mem_flags.coh == DEV_MEM_COHERENT && !RttEntriesInRangeCohDevMem(RttAt(old_s, walk_rtt_addr), walk_level, base, walk_top_pre) ==> ResultEqual(result, RMI_ERROR_RTT(walk_level as int)))
    && (!RttEntriesInRangeOutputContiguous(RttAt(old_s, walk_rtt_addr), walk_level, base, walk_top_pre, RecAt(old_s, rec_ptr).dev_mem_pa) ==> ResultEqual(result, RMI_ERROR_RTT(walk_level as int)))
    && (!AddrRangeIsAuxLive(old_s, base, top, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_RTT(walk_level as int)))
    && (ResultEqual(result, RMI_SUCCESS) ==> (RttEntriesInRangeRipas(RttAt(old_s, walk_rtt_addr), walk_level, base, walk_top_pre, DEV) && RecAt(old_s, rec_ptr).dev_mem_addr == MinAddress(top, walk_top_pre) && RecAt(old_s, rec_ptr).dev_mem_pa == RecAt(old_s, rec_ptr).dev_mem_pa + (walk_top_pre - base) && out_top == MinAddress(top, walk_top_pre)))
}