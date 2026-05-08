pub open spec fn RMI_RTT_SET_S2AP_spec(
    s: S,
    rd: Address,
    rec_ptr: Address,
    base: Address,
    top: Address,
) -> (result: RmiCommandReturnCode, out_top: Address, rtt_tree: u64)
{
    let realm = RealmAt(s, rd);
    let realm_pre = RealmAt(s, rd);
    let rec = RecAt(s, rec_ptr);
    let not_aligned = RttWalkAnyNotAligned(s, realm, base, top, RMM_RTT_PAGE_LEVEL);
    
    // Failure conditions - rd_align
    if !AddrIsGranuleAligned(rd) {
        (RMI_ERROR_INPUT, 0 as Address, 0)
    }
    // Failure conditions - rd_bound
    else if !PaIsDelegable(rd) {
        (RMI_ERROR_INPUT, 0 as Address, 0)
    }
    // Failure conditions - rd_state
    else if GranuleAt(s, rd).state != RD {
        (RMI_ERROR_INPUT, 0 as Address, 0)
    }
    // Failure conditions - rec_align
    else if !AddrIsGranuleAligned(rec_ptr) {
        (RMI_ERROR_INPUT, 0 as Address, 0)
    }
    // Failure conditions - rec_bound
    else if !PaIsDelegable(rec_ptr) {
        (RMI_ERROR_INPUT, 0 as Address, 0)
    }
    // Failure conditions - rec_gran_state
    else if GranuleAt(s, rec_ptr).state != REC {
        (RMI_ERROR_INPUT, 0 as Address, 0)
    }
    // Failure conditions - rec_state
    else if rec.state == REC_RUNNING {
        (RMI_ERROR_REC, 0 as Address, 0)
    }
    // Failure conditions - rec_owner
    else if rec.owner != rd {
        (RMI_ERROR_REC, 0 as Address, 0)
    }
    // Failure conditions - size_valid
    else if UInt(top) <= UInt(base) {
        (RMI_ERROR_INPUT, 0 as Address, 0)
    }
    // Failure conditions - base_bound
    else if base != rec.s2ap_addr {
        (RMI_ERROR_INPUT, 0 as Address, 0)
    }
    // Failure conditions - top_bound
    else if UInt(top) > UInt(rec.s2ap_top) {
        (RMI_ERROR_INPUT, 0 as Address, 0)
    }
    // Failure conditions - top_gran_align
    else if !AddrIsGranuleAligned(top) {
        (RMI_ERROR_INPUT, 0 as Address, 0)
    }
    // Failure conditions - base_align_pri
    else if (not_aligned.valid == RMM_TRUE
             && !AddrRangeIsWithin(
                base, top,
                AlignDownToRttLevel(s, not_aligned.addr, not_aligned.walk.level),
                AlignUpToRttLevel(s, not_aligned.addr, not_aligned.walk.level))
             && not_aligned.index == RMM_RTT_TREE_PRIMARY
             && not_aligned.walk.rtte.s2ap_indirect.overlay_index != rec.s2ap_overlay_index) {
        (RMI_ERROR_RTT, 0 as Address, not_aligned.walk.level as u64)
    }
    // Failure conditions - base_align_aux
    else if (not_aligned.valid == RMM_TRUE
             && !AddrRangeIsWithin(
                base, top,
                AlignDownToRttLevel(s, not_aligned.addr, not_aligned.walk.level),
                AlignUpToRttLevel(s, not_aligned.addr, not_aligned.walk.level))
             && not_aligned.index != RMM_RTT_TREE_PRIMARY
             && not_aligned.walk.rtte.s2ap_indirect.overlay_index != rec.s2ap_overlay_index) {
        (RMI_ERROR_RTT_AUX, 0 as Address, not_aligned.index as u64)
    }
    // Success condition
    else {
        (RMI_SUCCESS, rec.s2ap_addr, 0)
    }
}