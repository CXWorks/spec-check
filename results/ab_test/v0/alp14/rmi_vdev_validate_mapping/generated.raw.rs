```verus
pub open spec fn RMI_VDEV_VALIDATE_MAPPING_spec(s: S, rd: Address, rec_ptr: Address, pdev_ptr: Address, vdev_ptr: Address, base: Address, top: Address, result: RmiCommandReturnCode, out_top: Address) -> bool {
    let realm_pre = RealmAt(rd);
    let rec = RecAt(rec_ptr);
    let pdev = PdevAt(pdev_ptr);
    let vdev = VdevAt(vdev_ptr);
    let pa_pre = rec.dev_mem_pa;
    let walk = RttWalk(RealmAt(rd), base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let walk_top_pre = RttSkipEntriesWithRipas(RttAt(walk.rtt_addr), walk.level, base, top, false);
    
    // Failure conditions
    let rd_align_fail = !AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let rd_bound_fail = !PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let rd_state_fail = GranuleAt(rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT);
    let rec_align_fail = !AddrIsGranuleAligned(rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let rec_bound_fail = !PaIsDelegable(rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let rec_gran_state_fail = GranuleAt(rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT);
    let rec_state_fail = rec.state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC);
    let rec_owner_fail = rec.owner != rd ==> ResultEqual(result, RMI_ERROR_REC);
    let pdev_align_fail = !AddrIsGranuleAligned(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let pdev_bound_fail = !PaIsDelegable(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let pdev_gran_state_fail = GranuleAt(pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT);
    let vdev_align_fail = !AddrIsGranuleAligned(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let vdev_bound_fail = !PaIsDelegable(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let vdev_gran_state_fail = GranuleAt(vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT);
    let vdev_pdev_fail = vdev.pdev != pdev_ptr ==> ResultEqual(result, RMI_ERROR_DEVICE);
    let size_valid_fail = UInt(top) <= UInt(base) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let base_bound_fail = base != rec.dev_mem_addr ==> ResultEqual(result, RMI_ERROR_INPUT);
    let top_bound_fail = UInt(top) > UInt(rec.dev_mem_top) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let base_align_fail = !AddrIsRttLevelAligned(base, walk.level) ==> ResultEqual(result, RMI_ERROR_RTT);
    let top_gran_align_fail = !AddrIsGranuleAligned(top) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let no_progress_fail = UInt(base) == UInt(walk_top_pre) ==> ResultEqual(result, RMI_ERROR_RTT);
    let ncoh_attr_fail = (rec.dev_mem_flags.coh == DEV_MEM_NON_COHERENT && !RttEntriesInRangeMemAttr(RttAt(walk.rtt_addr), walk.level, base, walk_top_pre, MEMATTR_NON_CACHEABLE)) ==> ResultEqual(result, RMI_ERROR_RTT);
    let ncoh_pa_fail = (rec.dev_mem_flags.coh == DEV_MEM_NON_COHERENT && !RttEntriesInRangeNonCohDevMem(RttAt(walk.rtt_addr), walk.level, base, walk_top_pre)) ==> ResultEqual(result, RMI_ERROR_RTT);
    let coh_attr_fail = (rec.dev_mem_flags.coh == DEV_MEM_COHERENT && !RttEntriesInRangeMemAttr(RttAt(walk.rtt_addr), walk.level, base, walk_top_pre, MEMATTR_PASSTHROUGH)) ==> ResultEqual(result, RMI_ERROR_RTT);
    let coh_pa_fail = (rec.dev_mem_flags.coh == DEV_MEM_COHERENT && !RttEntriesInRangeCohDevMem(RttAt(walk.rtt_addr), walk.level, base, walk_top_pre)) ==> ResultEqual(result, RMI_ERROR_RTT);
    let linear_map_fail = !RttEntriesInRangeOutputContiguous(RttAt(walk.rtt_addr), walk.level, base, walk_top_pre, rec.dev_mem_pa) ==> ResultEqual(result, RMI_ERROR_RTT);
    let aux_live_fail = AddrRangeIsAuxLive(base, top, realm_pre) ==> ResultEqual(result, RMI_ERROR_RTT);
    
    // Success conditions
    let rtte_ripas_success = result.is_Ok() ==> RttEntriesInRangeRipas(RttAt(walk.rtt_addr), walk.level, base, walk_top_pre, DEV);
    let dev_mem_addr_success = result.is_Ok() ==> rec.dev_mem_addr == MinAddress(top, walk_top_pre);
    let dev_mem_pa_success = result.is_Ok() ==> rec.dev_mem_pa == ToAddress(UInt(pa_pre) + (UInt(walk_top_pre) - UInt(base)));
    let out_top_success = result.is_Ok() ==> out_top == MinAddress(top, walk_top_pre);
    
    rd_align_fail && rd_bound_fail && rd_state_fail && rec_align_fail && rec_bound_fail && rec_gran_state_fail && rec_state_fail && rec_owner_fail && pdev_align_fail && pdev_bound_fail && pdev_gran_state_fail && vdev_align_fail && vdev_bound_fail && vdev_gran_state_fail && vdev_pdev_fail && size_valid_fail && base_bound_fail && top_bound_fail && base_align_fail && top_gran_align_fail && no_progress_fail && ncoh_attr_fail && ncoh_pa_fail && coh_attr_fail && coh_pa_fail && linear_map_fail && aux_live_fail && rtte_ripas_success && dev_mem_addr_success && dev_mem_pa_success && out_top_success
}
```