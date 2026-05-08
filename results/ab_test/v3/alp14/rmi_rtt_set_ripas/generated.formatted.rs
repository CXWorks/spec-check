pub open spec fn RMI_RTT_SET_RIPAS_spec(
    old_s: S,
    new_s: S,
    rd: Address,
    rec_ptr: Address,
    base: Address,
    top: Address,
    result: Result<(Address), RmiStatusCode>,
) -> bool {
    let realm_pre = RealmAt(old_s, rd);
    let realm = RealmAt(new_s, rd);
    let rec_pre = RecAt(old_s, rec_ptr);
    let rec = RecAt(new_s, rec_ptr);
    let walk = RttWalk(old_s, realm_pre, base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let ripas_pre = walk.rtte.ripas;
    let walk_top_pre = RttSkipEntriesWithRipas(
        old_s,
        RttAt(old_s, walk.rtt_addr),
        walk.level,
        base,
        top,
        (rec_pre.ripas_value == RAM) && (rec_pre.ripas_destroyed != CHANGE_DESTROYED),
    );

    // Failure conditions
    let rd_align_fail = !AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let rd_bound_fail = !PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let rd_state_fail = GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT);
    let rec_align_fail = !AddrIsGranuleAligned(rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let rec_bound_fail = !PaIsDelegable(rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let rec_gran_state_fail = GranuleAt(old_s, rec_ptr).state != REC ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );
    let rec_state_fail = rec_pre.state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC);
    let rec_owner_fail = rec_pre.owner != rd ==> ResultEqual(result, RMI_ERROR_REC);
    let size_valid_fail = UInt(top) <= UInt(base) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let base_bound_fail = base != rec_pre.ripas_addr ==> ResultEqual(result, RMI_ERROR_INPUT);
    let top_bound_fail = UInt(top) > UInt(rec_pre.ripas_top) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );
    let top_gran_align_fail = !AddrIsGranuleAligned(top) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let base_align_fail = (!AddrIsRttLevelAligned(base, walk.level) && ripas_pre
        != rec_pre.ripas_value) ==> ResultEqual(result, RMI_ERROR_RTT);
    let no_progress_fail = (UInt(base) == UInt(walk_top_pre) && ripas_pre != rec_pre.ripas_value)
        ==> ResultEqual(result, RMI_ERROR_RTT);
    let aux_live_fail = AddrRangeIsAuxLive(old_s, base, top, realm_pre) ==> ResultEqual(
        result,
        RMI_ERROR_RTT,
    );

    // Success conditions
    let rtte_ripas_success = result.is_Ok() ==> RttEntriesInRangeRipas(
        old_s,
        RttAt(new_s, walk.rtt_addr),
        walk.level,
        base,
        walk_top_pre,
        rec.ripas_value,
    );
    let ripas_addr_success = result.is_Ok() ==> rec.ripas_addr == MinAddress(top, walk_top_pre);
    let out_top_success = result.is_Ok() ==> result.get_Ok_0().0 == MinAddress(top, walk_top_pre);

    rd_align_fail && rd_bound_fail && rd_state_fail && rec_align_fail && rec_bound_fail
        && rec_gran_state_fail && rec_state_fail && rec_owner_fail && size_valid_fail
        && base_bound_fail && top_bound_fail && top_gran_align_fail && base_align_fail
        && no_progress_fail && aux_live_fail && rtte_ripas_success && ripas_addr_success
        && out_top_success
}