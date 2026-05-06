```verus
pub open spec fn RMI_VDEV_VALIDATE_MAPPING_spec(
    old_s: S,
    new_s: S,
    rd: Address,
    rec_ptr: Address,
    pdev_ptr: Address,
    vdev_ptr: Address,
    base: Address,
    top: Address,
    result: Result<(Address,), RmiStatusCode>,
) -> bool {
    let realm = RealmAt(old_s, rd);
    let realm_pre = RealmAt(old_s, rd);
    let rec = RecAt(old_s, rec_ptr);
    let pdev = PdevAt(old_s, pdev_ptr);
    let vdev = VdevAt(old_s, vdev_ptr);
    let pa_pre = rec.dev_mem_pa;
    let walk = RttWalk(old_s, realm, base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let walk_top_pre = RttSkipEntriesWithRipas(old_s, RttAt(old_s, walk.rtt_addr), walk.level, base, top, false);

    // Failure conditions
    ((!AddrIsGranuleAligned(rd)) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((!PaIsDelegable(rd)) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((GranuleAt(old_s, rd).state != RD) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((!AddrIsGranuleAligned(rec_ptr)) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((!PaIsDelegable(rec_ptr)) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((GranuleAt(old_s, rec_ptr).state != REC) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((rec.state == REC_RUNNING) ==> ResultEqual(result, RMI_ERROR_REC)) &&
    ((rec.owner != rd) ==> ResultEqual(result, RMI_ERROR_REC)) &&
    ((!AddrIsGranuleAligned(pdev_ptr)) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((!PaIsDelegable(pdev_ptr)) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((GranuleAt(old_s, pdev_ptr).state != PDEV) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((!AddrIsGranuleAligned(vdev_ptr)) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((!PaIsDelegable(vdev_ptr)) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((GranuleAt(old_s, vdev_ptr).state != VDEV) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((vdev.pdev != pdev_ptr) ==> ResultEqual(result, RMI_ERROR_DEVICE)) &&
    ((UInt(top) <= UInt(base)) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((base != rec.dev_mem_addr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((UInt(top) > UInt(rec.dev_mem_top)) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((!AddrIsRttLevelAligned(base, walk.level)) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((!AddrIsGranuleAligned(top)) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((UInt(base) == UInt(walk_top_pre)) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (((rec.dev_mem_flags.coh == DEV_MEM_NON_COHERENT) && !RttEntriesInRangeMemAttr(old_s, RttAt(old_s, walk.rtt_addr), walk.level, base, walk_top_pre, MEMATTR_NON_CACHEABLE)) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (((rec.dev_mem_flags.coh == DEV_MEM_NON_COHERENT) && !RttEntriesInRangeNonCohDevMem(old_s, RttAt(old_s, walk.rtt_addr), walk.level, base, walk_top_pre)) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (((rec.dev_mem_flags.coh == DEV_MEM_COHERENT) && !RttEntriesInRangeMemAttr(old_s, RttAt(old_s, walk.rtt_addr), walk.level, base, walk_top_pre, MEMATTR_PASSTHROUGH)) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (((rec.dev_mem_flags.coh == DEV_MEM_COHERENT) && !RttEntriesInRangeCohDevMem(old_s, RttAt(old_s, walk.rtt_addr), walk.level, base, walk_top_pre)) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((!RttEntriesInRangeOutputContiguous(old_s, RttAt(old_s, walk.rtt_addr), walk.level, base, walk_top_pre, rec.dev_mem_pa)) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((AddrRangeIsAuxLive(old_s, base, top, realm_pre)) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&

    // Success conditions
    (result.is_Ok() ==> (
        RttEntriesInRangeRipas(new_s, RttAt(new_s, walk.rtt_addr), walk.level, base, walk_top_pre, DEV) &&
        RecAt(new_s, rec_ptr).dev_mem_addr == MinAddress(top, walk_top_pre) &&
        RecAt(new_s, rec_ptr).dev_mem_pa == ToAddress(UInt(pa_pre) + (UInt(walk_top_pre) - UInt(base))) &&
        result.get_Ok_0().0 == MinAddress(top, walk_top_pre)
    ))
}
```