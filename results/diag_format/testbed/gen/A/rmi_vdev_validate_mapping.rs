pub open spec fn rmi_vdev_validate_mapping_spec(rd: Address, rec_ptr: Address, pdev_ptr: Address, vdev_ptr: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S) -> bool {
    let realm_pre = RealmAt(old_s, rd);
    let rec = RecAt(old_s, rec_ptr);
    let rec_post = RecAt(new_s, rec_ptr);
    let vdev = VdevAt(old_s, vdev_ptr);
    let pa_pre = rec.dev_mem_pa;
    let walk = RttWalk(old_s, realm_pre, base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let rtt_pre = RttAt(old_s, walk.rtt_addr);
    let walk_top_pre = RttSkipEntriesWithRipas(old_s, rtt_pre, walk.level, base, top, false);
    let rd_ok = AddrIsGranuleAligned(old_s, rd) && PaIsDelegable(old_s, rd) && GranuleAt(old_s, rd).state == RD;
    let rec_ok = AddrIsGranuleAligned(old_s, rec_ptr) && PaIsDelegable(old_s, rec_ptr) && GranuleAt(old_s, rec_ptr).state == REC;
    let pdev_ok = AddrIsGranuleAligned(old_s, pdev_ptr) && PaIsDelegable(old_s, pdev_ptr) && GranuleAt(old_s, pdev_ptr).state == PDEV;
    let vdev_ok = AddrIsGranuleAligned(old_s, vdev_ptr) && PaIsDelegable(old_s, vdev_ptr) && GranuleAt(old_s, vdev_ptr).state == VDEV;
    let rec_state_ok = rec_ok && rec.state != REC_RUNNING && rec.owner == rd;
    let ctx_ok = rd_ok && rec_state_ok && pdev_ok && vdev_ok && vdev.pdev == pdev_ptr;
    let range_ok = top > base && base == rec.dev_mem_addr && top <= rec.dev_mem_top && AddrIsGranuleAligned(old_s, top);
    let non_coh = rec.dev_mem_flags.coh == DEV_MEM_NON_COHERENT;
    let coh = rec.dev_mem_flags.coh == DEV_MEM_COHERENT;
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((rd_ok && rec_ok && rec.state == REC_RUNNING) ==> ResultEqual(result, RMI_ERROR_REC))
    && ((rd_ok && rec_ok && rec.owner != rd) ==> ResultEqual(result, RMI_ERROR_REC))
    && (!AddrIsGranuleAligned(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((rd_ok && rec_state_ok && pdev_ok && vdev_ok && vdev.pdev != pdev_ptr) ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && ((ctx_ok && top <= base) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((ctx_ok && base != rec.dev_mem_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((ctx_ok && top > rec.dev_mem_top) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((ctx_ok && !AddrIsGranuleAligned(old_s, top)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((ctx_ok && range_ok && !AddrIsRttLevelAligned(old_s, base, walk.level)) ==> ResultEqual(result, RMI_ERROR_RTT(walk.level)))
    && ((ctx_ok && range_ok && AddrIsRttLevelAligned(old_s, base, walk.level) && walk_top_pre == base) ==> ResultEqual(result, RMI_ERROR_RTT(walk.level)))
    && ((ctx_ok && range_ok && AddrIsRttLevelAligned(old_s, base, walk.level) && walk_top_pre != base && non_coh
            && !RttEntriesInRangeMemAttr(old_s, rtt_pre, walk.level, base, walk_top_pre, MEMATTR_NON_CACHEABLE)) ==> ResultEqual(result, RMI_ERROR_RTT(walk.level)))
    && ((ctx_ok && range_ok && AddrIsRttLevelAligned(old_s, base, walk.level) && walk_top_pre != base && non_coh
            && !RttEntriesInRangeNonCohDevMem(old_s, rtt_pre, walk.level, base, walk_top_pre)) ==> ResultEqual(result, RMI_ERROR_RTT(walk.level)))
    && ((ctx_ok && range_ok && AddrIsRttLevelAligned(old_s, base, walk.level) && walk_top_pre != base && coh
            && !RttEntriesInRangeMemAttr(old_s, rtt_pre, walk.level, base, walk_top_pre, MEMATTR_PASSTHROUGH)) ==> ResultEqual(result, RMI_ERROR_RTT(walk.level)))
    && ((ctx_ok && range_ok && AddrIsRttLevelAligned(old_s, base, walk.level) && walk_top_pre != base && coh
            && !RttEntriesInRangeCohDevMem(old_s, rtt_pre, walk.level, base, walk_top_pre)) ==> ResultEqual(result, RMI_ERROR_RTT(walk.level)))
    && ((ctx_ok && range_ok && AddrIsRttLevelAligned(old_s, base, walk.level) && walk_top_pre != base
            && !RttEntriesInRangeOutputContiguous(old_s, rtt_pre, walk.level, base, walk_top_pre, pa_pre)) ==> ResultEqual(result, RMI_ERROR_RTT(walk.level)))
    && ((ctx_ok && range_ok && AddrRangeIsAuxLive(old_s, base, top, realm_pre)) ==> ResultEqual(result, RMI_ERROR_RTT(walk.level)))
    && (result.is_Err() ==> new_s == old_s)
    && ((ctx_ok && range_ok
            && AddrIsRttLevelAligned(old_s, base, walk.level)
            && walk_top_pre != base
            && (non_coh ==> (RttEntriesInRangeMemAttr(old_s, rtt_pre, walk.level, base, walk_top_pre, MEMATTR_NON_CACHEABLE)
                             && RttEntriesInRangeNonCohDevMem(old_s, rtt_pre, walk.level, base, walk_top_pre)))
            && (coh ==> (RttEntriesInRangeMemAttr(old_s, rtt_pre, walk.level, base, walk_top_pre, MEMATTR_PASSTHROUGH)
                         && RttEntriesInRangeCohDevMem(old_s, rtt_pre, walk.level, base, walk_top_pre)))
            && RttEntriesInRangeOutputContiguous(old_s, rtt_pre, walk.level, base, walk_top_pre, pa_pre)
            && !AddrRangeIsAuxLive(old_s, base, top, realm_pre))
        ==> (result.is_Ok()
            && RttEntriesInRangeRipas(new_s, RttAt(new_s, walk.rtt_addr), walk.level, base, walk_top_pre, DEV)
            && rec_post.dev_mem_addr == MinAddress(old_s, top, walk_top_pre)
            && (rec_post.dev_mem_pa as int) == (pa_pre as int) + ((walk_top_pre as int) - (base as int))
            && out_top == MinAddress(old_s, top, walk_top_pre)))
}
