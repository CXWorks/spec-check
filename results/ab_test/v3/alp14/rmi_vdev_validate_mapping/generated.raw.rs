```verus
pub open spec fn rmi_vdev_validate_mapping_spec(
    result: RmiCommandReturnCode,
    out_top: Address,
    rd: Address,
    rec_ptr: Address,
    pdev_ptr: Address,
    vdev_ptr: Address,
    base: Address,
    top: Address,
    old_s: S,
    new_s: S
) -> bool {
    let realm = RealmAt(old_s, rd);
    let realm_pre = RealmAt(old_s, rd);
    let rec = RecAt(old_s, rec_ptr);
    let pdev = PdevAt(old_s, pdev_ptr);
    let vdev = VdevAt(old_s, vdev_ptr);
    let pa_pre = rec.dev_mem_pa;
    let walk = RttWalk(old_s, realm, base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let walk_top_pre = RttSkipEntriesWithRipas(
        RttAt(old_s, walk.rtt_addr),
        walk.level,
        base, top,
        false);
    
    (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (rec.state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
    && (rec.owner != rd ==> ResultEqual(result, RMI_ERROR_REC))
    && (!AddrIsGranuleAligned(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (vdev.pdev != pdev_ptr ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (UInt(top) <= UInt(base) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (base != rec.dev_mem_addr ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (UInt(top) > UInt(rec.dev_mem_top) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(base, walk.level) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(top) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (UInt(base) == UInt(walk_top_pre) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((rec.dev_mem_flags.coh == DEV_MEM_NON_COHERENT
        && !RttEntriesInRangeMemAttr(
            RttAt(old_s, walk.rtt_addr),
            walk.level,
            base, walk_top_pre,
            MEMATTR_NON_CACHEABLE))
        ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((rec.dev_mem_flags.coh == DEV_MEM_NON_COHERENT
        && !RttEntriesInRangeNonCohDevMem(
            RttAt(old_s, walk.rtt_addr),
            walk.level,
            base, walk_top_pre))
        ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((rec.dev_mem_flags.coh == DEV_MEM_COHERENT
        && !RttEntriesInRangeMemAttr(
            RttAt(old_s, walk.rtt_addr),
            walk.level,
            base, walk_top_pre,
            MEMATTR_PASSTHROUGH))
        ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((rec.dev_mem_flags.coh == DEV_MEM_COHERENT
        && !RttEntriesInRangeCohDevMem(
            RttAt(old_s, walk.rtt_addr),
            walk.level,
            base, walk_top_pre))
        ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RttEntriesInRangeOutputContiguous(
            RttAt(old_s, walk.rtt_addr),
            walk.level,
            base, walk_top_pre,
            rec.dev_mem_pa)
        ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (AddrRangeIsAuxLive(old_s, base, top, realm_pre) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((AddrIsGranuleAligned(rd)
        && PaIsDelegable(rd)
        && GranuleAt(old_s, rd).state == RD
        && AddrIsGranuleAligned(rec_ptr)
        && PaIsDelegable(rec_ptr)
        && GranuleAt(old_s, rec_ptr).state == REC
        && rec.state != REC_RUNNING
        && rec.owner == rd
        && AddrIsGranuleAligned(pdev_ptr)
        && PaIsDelegable(pdev_ptr)
        && GranuleAt(old_s, pdev_ptr).state == PDEV
        && AddrIsGranuleAligned(vdev_ptr)
        && PaIsDelegable(vdev_ptr)
        && GranuleAt(old_s, vdev_ptr).state == VDEV
        && vdev.pdev == pdev_ptr
        && UInt(top) > UInt(base)
        && base == rec.dev_mem_addr
        && UInt(top) <= UInt(rec.dev_mem_top)
        && AddrIsRttLevelAligned(base, walk.level)
        && AddrIsGranuleAligned(top)
        && UInt(base) != UInt(walk_top_pre)
        && (rec.dev_mem_flags.coh != DEV_MEM_NON_COHERENT
            || RttEntriesInRangeMemAttr(
                RttAt(old_s, walk.rtt_addr),
                walk.level,
                base, walk_top_pre,
                MEMATTR_NON_CACHEABLE))
        && (rec.dev_mem_flags.coh != DEV_MEM_NON_COHERENT
            || RttEntriesInRangeNonCohDevMem(
                RttAt(old_s, walk.rtt_addr),
                walk.level,
                base, walk_top_pre))
        && (rec.dev_mem_flags.coh != DEV_MEM_COHERENT
            || RttEntriesInRangeMemAttr(
                RttAt(old_s, walk.rtt_addr),