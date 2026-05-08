```verus
pub open spec fn rmi_vdev_validate_mapping_spec(
    result: RmiCommandReturnCode,
    out_top: Address,
    old_s: S,
    new_s: S
) -> bool {
    let rd = old_s.rd;
    let rec_ptr = old_s.rec_ptr;
    let pdev_ptr = old_s.pdev_ptr;
    let vdev_ptr = old_s.vdev_ptr;
    let base = old_s.base;
    let top = old_s.top;
    
    let realm_pre = RealmAt(old_s, rd);
    let rec = RecAt(old_s, rec_ptr);
    let pdev = PdevAt(old_s, pdev_ptr);
    let vdev = VdevAt(old_s, vdev_ptr);
    let pa_pre = rec.dev_mem_pa;
    let walk = RttWalk(old_s, realm_pre, base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let walk_top_pre = RttSkipEntriesWithRipas(
        RttAt(old_s, walk.rtt_addr),
        walk.level,
        base, top,
        false
    );
    
    // Failure conditions
    let rd_align_fail = !result.is_Ok() && !AddrIsGranuleAligned(rd) && result.is_Err() && result.get_Err_0() == RMI_ERROR_INPUT;
    let rd_bound_fail = !result.is_Ok() && !PaIsDelegable(rd) && result.is_Err() && result.get_Err_0() == RMI_ERROR_INPUT;
    let rd_state_fail = !result.is_Ok() && GranuleAt(old_s, rd).state != RD && result.is_Err() && result.get_Err_0() == RMI_ERROR_INPUT;
    let rec_align_fail = !result.is_Ok() && !AddrIsGranuleAligned(rec_ptr) && result.is_Err() && result.get_Err_0() == RMI_ERROR_INPUT;
    let rec_bound_fail = !result.is_Ok() && !PaIsDelegable(rec_ptr) && result.is_Err() && result.get_Err_0() == RMI_ERROR_INPUT;
    let rec_gran_state_fail = !result.is_Ok() && GranuleAt(old_s, rec_ptr).state != REC && result.is_Err() && result.get_Err_0() == RMI_ERROR_INPUT;
    let rec_state_fail = !result.is_Ok() && rec.state == REC_RUNNING && result.is_Err() && result.get_Err_0() == RMI_ERROR_REC;
    let rec_owner_fail = !result.is_Ok() && rec.owner != rd && result.is_Err() && result.get_Err_0() == RMI_ERROR_REC;
    let pdev_align_fail = !result.is_Ok() && !AddrIsGranuleAligned(pdev_ptr) && result.is_Err() && result.get_Err_0() == RMI_ERROR_INPUT;
    let pdev_bound_fail = !result.is_Ok() && !PaIsDelegable(pdev_ptr) && result.is_Err() && result.get_Err_0() == RMI_ERROR_INPUT;
    let pdev_gran_state_fail = !result.is_Ok() && GranuleAt(old_s, pdev_ptr).state != PDEV && result.is_Err() && result.get_Err_0() == RMI_ERROR_INPUT;
    let vdev_align_fail = !result.is_Ok() && !AddrIsGranuleAligned(vdev_ptr) && result.is_Err() && result.get_Err_0() == RMI_ERROR_INPUT;
    let vdev_bound_fail = !result.is_Ok() && !PaIsDelegable(vdev_ptr) && result.is_Err() && result.get_Err_0() == RMI_ERROR_INPUT;
    let vdev_gran_state_fail = !result.is_Ok() && GranuleAt(old_s, vdev_ptr).state != VDEV && result.is_Err() && result.get_Err_0() == RMI_ERROR_INPUT;
    let vdev_pdev_fail = !result.is_Ok() && vdev.pdev != pdev_ptr && result.is_Err() && result.get_Err_0() == RMI_ERROR_DEVICE;
    let size_valid_fail = !result.is_Ok() && UInt(top) <= UInt(base) && result.is_Err() && result.get_Err_0() == RMI_ERROR_INPUT;
    let base_bound_fail = !result.is_Ok() && base != rec.dev_mem_addr && result.is_Err() && result.get_Err_0() == RMI_ERROR_INPUT;
    let top_bound_fail = !result.is_Ok() && UInt(top) > UInt(rec.dev_mem_top) && result.is_Err() && result.get_Err_0() == RMI_ERROR_INPUT;
    let base_align_fail = !result.is_Ok() && !AddrIsRttLevelAligned(base, walk.level) && result.is_Err() && result.get_Err_0() == RMI_ERROR_RTT;
    let top_gran_align_fail = !result.is_Ok() && !AddrIsGranuleAligned(top) && result.is_Err() && result.get_Err_0() == RMI_ERROR_INPUT;
    let no_progress_fail = !result.is_Ok() && UInt(base) == UInt(walk_top_pre) && result.is_Err() && result.get_Err_0() == RMI_ERROR_RTT;
    
    let ncoh_attr_fail = !result.is_Ok() && 
        (rec.dev_mem_flags.coh == DEV_MEM_NON_COHERENT &&
         !RttEntriesInRangeMemAttr(
             RttAt(old_s, walk.rtt_addr),
             walk.level,
             base, walk_top_pre,
             MEMATTR_NON_CACHEABLE)) &&
        result.is_Err() && result.get_Err_0() == RMI_ERROR_RTT;
    
    let ncoh_pa_fail = !result.is_Ok() &&
        (rec.dev_mem_flags.coh == DEV_MEM_NON_COHERENT &&
         !RttEntriesInRangeNonCohDevMem(
             RttAt(old_s, walk.rtt_addr),
             walk.level,
             base, walk_top_pre)) &&
        result.is_Err() && result.get_Err_0() == RMI_ERROR_RTT;
    
    let coh_attr_fail = !result.is_Ok() &&
        (rec.dev_mem_flags.coh == DEV_MEM_COHERENT &&
         !RttEntriesInRangeMemAttr(
             RttAt(old_s, walk.rtt_addr),
             walk.level,
             base, walk_top_pre,
             MEMATTR_PASSTHROUGH)) &&
        result.is_Err() && result.get_Err_0() == RMI_ERROR_RTT;
    
    let coh_pa_fail = !result.is_Ok() &&
        (rec.dev_mem_flags.coh == DEV_MEM_COHERENT &&
         !RttEntriesInRangeCohDevMem(
             RttAt(old_s, walk.rtt_addr),
             walk.level,