pub open spec fn rmi_vdev_validate_mapping_spec(rd: Address, rec_ptr: Address, pdev_ptr: Address, vdev_ptr: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S) -> bool {
    let realm_pre = RealmAt(old_s, rd);
    let rec_pre = RecAt(old_s, rec_ptr);
    let vdev_pre = VdevAt(old_s, vdev_ptr);
    let walk = RttWalk(old_s, realm_pre, base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let walk_top_pre = RttSkipEntriesWithRipas(old_s, RttAt(old_s, walk.rtt_addr), walk.level, base, top, false);
    let pa_pre = rec_pre.dev_mem_pa;
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (rec_pre.state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
    && (rec_pre.owner != rd ==> ResultEqual(result, RMI_ERROR_REC))
    && (!AddrIsGranuleAligned(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (vdev_pre.pdev != pdev_ptr ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (top <= base ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (base != rec_pre.dev_mem_addr ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (top > rec_pre.dev_mem_top ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(old_s, base, walk.level as int) ==> ResultEqual(result, RMI_ERROR_RTT(walk.level as int)))
    && (!AddrIsGranuleAligned(old_s, top) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (base == walk_top_pre ==> ResultEqual(result, RMI_ERROR_RTT(walk.level as int)))
    && ((rec_pre.dev_mem_flags.coh == DEV_MEM_NON_COHERENT
            && !RttEntriesInRangeMemAttr(old_s, RttAt(old_s, walk.rtt_addr), walk.level as int, base, walk_top_pre, MEMATTR_NON_CACHEABLE))
        ==> ResultEqual(result, RMI_ERROR_RTT(walk.level as int)))
    && ((rec_pre.dev_mem_flags.coh == DEV_MEM_NON_COHERENT
            && !RttEntriesInRangeNonCohDevMem(old_s, RttAt(old_s, walk.rtt_addr), walk.level as int, base, walk_top_pre))
        ==> ResultEqual(result, RMI_ERROR_RTT(walk.level as int)))
    && ((rec_pre.dev_mem_flags.coh == DEV_MEM_COHERENT
            && !RttEntriesInRangeMemAttr(old_s, RttAt(old_s, walk.rtt_addr), walk.level as int, base, walk_top_pre, MEMATTR_PASSTHROUGH))
        ==> ResultEqual(result, RMI_ERROR_RTT(walk.level as int)))
    && ((rec_pre.dev_mem_flags.coh == DEV_MEM_COHERENT
            && !RttEntriesInRangeCohDevMem(old_s, RttAt(old_s, walk.rtt_addr), walk.level as int, base, walk_top_pre))
        ==> ResultEqual(result, RMI_ERROR_RTT(walk.level as int)))
    && (!RttEntriesInRangeOutputContiguous(old_s, RttAt(old_s, walk.rtt_addr), walk.level as int, base, walk_top_pre, rec_pre.dev_mem_pa)
        ==> ResultEqual(result, RMI_ERROR_RTT(walk.level as int)))
    && (AddrRangeIsAuxLive(old_s, base, top, realm_pre) ==> ResultEqual(result, RMI_ERROR_RTT(walk.level as int)))
    && ((AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && GranuleAt(old_s, rd).state == RD
        && AddrIsGranuleAligned(old_s, rec_ptr)
        && PaIsDelegable(old_s, rec_ptr)
        && GranuleAt(old_s, rec_ptr).state == REC
        && rec_pre.state != REC_RUNNING
        && rec_pre.owner == rd
        && AddrIsGranuleAligned(old_s, pdev_ptr)
        && PaIsDelegable(old_s, pdev_ptr)
        && GranuleAt(old_s, pdev_ptr).state == PDEV
        && AddrIsGranuleAligned(old_s, vdev_ptr)
        && PaIsDelegable(old_s, vdev_ptr)
        && GranuleAt(old_s, vdev_ptr).state == VDEV
        && vdev_pre.pdev == pdev_ptr
        && top > base
        && base == rec_pre.dev_mem_addr
        && top <= rec_pre.dev_mem_top
        && AddrIsRttLevelAligned(old_s, base, walk.level as int)
        && AddrIsGranuleAligned(old_s, top)
        && base != walk_top_pre
        && (rec_pre.dev_mem_flags.coh == DEV_MEM_NON_COHERENT
            ==> (RttEntriesInRangeMemAttr(old_s, RttAt(old_s, walk.rtt_addr), walk.level as int, base, walk_top_pre, MEMATTR_NON_CACHEABLE)
                && RttEntriesInRangeNonCohDevMem(old_s, RttAt(old_s, walk.rtt_addr), walk.level as int, base, walk_top_pre)))
        && (rec_pre.dev_mem_flags.coh == DEV_MEM_COHERENT
            ==> (RttEntriesInRangeMemAttr(old_s, RttAt(old_s, walk.rtt_addr), walk.level as int, base, walk_top_pre, MEMATTR_PASSTHROUGH)
                && RttEntriesInRangeCohDevMem(old_s, RttAt(old_s, walk.rtt_addr), walk.level as int, base, walk_top_pre)))
        && RttEntriesInRangeOutputContiguous(old_s, RttAt(old_s, walk.rtt_addr), walk.level as int, base, walk_top_pre, rec_pre.dev_mem_pa)
        && !AddrRangeIsAuxLive(old_s, base, top, realm_pre))
        ==> (result.is_Ok()
            && RttEntriesInRangeRipas(new_s, RttAt(new_s, walk.rtt_addr), walk.level as int, base, walk_top_pre, DEV)
            && RecAt(new_s, rec_ptr).dev_mem_addr == MinAddress(new_s, top, walk_top_pre)
            && RecAt(new_s, rec_ptr).dev_mem_pa == ToAddress(pa_pre as int + (walk_top_pre as int - base as int))
            && out_top == MinAddress(new_s, top, walk_top_pre)))
}