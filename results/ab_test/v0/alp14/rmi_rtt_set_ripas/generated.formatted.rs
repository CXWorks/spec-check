pub open spec fn RMI_RTT_SET_RIPAS_spec(
    s: S,
    rd: Address,
    rec_ptr: Address,
    base: Address,
    top: Address,
    result: Result<Address, RmiStatusCode>,
) -> bool {
    let realm = RealmAt(s, rd);
    let realm_pre = realm;
    let rec = RecAt(s, rec_ptr);
    let walk = RttWalk(s, realm, base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let ripas_pre = walk.rtte.ripas;
    let walk_top_pre = RttSkipEntriesWithRipas(
        s,
        RttAt(s, walk.rtt_addr),
        walk.level,
        base,
        top,
        (rec.ripas_value == RMM_RIPAS_RAM) && (rec.ripas_destroyed != RMM_RIPAS_CHANGE_DESTROYED),
    );

    let rd_align_fail = !AddrIsGranuleAligned(s, rd);
    let rd_bound_fail = !PaIsDelegable(s, rd);
    let rd_state_fail = GranuleAt(s, rd).state != RMM_GRANULE_STATE_RD;
    let rec_align_fail = !AddrIsGranuleAligned(s, rec_ptr);
    let rec_bound_fail = !PaIsDelegable(s, rec_ptr);
    let rec_gran_state_fail = GranuleAt(s, rec_ptr).state != RMM_GRANULE_STATE_REC;
    let rec_state_fail = rec.state == RMM_REC_STATE_REC_RUNNING;
    let rec_owner_fail = rec.owner != rd;
    let size_valid_fail = UInt(s, top) <= UInt(s, base);
    let base_bound_fail = base != rec.ripas_addr;
    let top_bound_fail = UInt(s, top) > UInt(s, rec.ripas_top);
    let base_align_fail = !AddrIsRttLevelAligned(s, base, walk.level) && ripas_pre
        != rec.ripas_value;
    let top_gran_align_fail = !AddrIsGranuleAligned(s, top);
    let no_progress_fail = (UInt(s, base) == UInt(s, walk_top_pre)) && ripas_pre != rec.ripas_value;
    let aux_live_fail = AddrRangeIsAuxLive(s, base, top, realm_pre);

    let failure_conditions = ((rd_align_fail ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    rd_bound_fail ==> ResultEqual(result, RMI_ERROR_INPUT)) && (rd_state_fail ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (rec_align_fail ==> ResultEqual(result, RMI_ERROR_INPUT)) && (rec_bound_fail
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (rec_gran_state_fail ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (rec_state_fail ==> ResultEqual(result, RMI_ERROR_REC)) && (rec_owner_fail
        ==> ResultEqual(result, RMI_ERROR_REC)) && (size_valid_fail ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (base_bound_fail ==> ResultEqual(result, RMI_ERROR_INPUT)) && (top_bound_fail
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (top_gran_align_fail ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (base_align_fail ==> result.is_Err() && result.get_Err_0() == RMI_ERROR_RTT) && (
    no_progress_fail ==> result.is_Err() && result.get_Err_0() == RMI_ERROR_RTT) && (aux_live_fail
        ==> result.is_Err() && result.get_Err_0() == RMI_ERROR_RTT));

    let success_conditions = (result.is_Ok() ==> (RttEntriesInRangeRipas(
        s,
        RttAt(s, walk.rtt_addr),
        walk.level,
        base,
        walk_top_pre,
        rec.ripas_value,
    ) && rec.ripas_addr == MinAddress(s, top, walk_top_pre) && result.get_Ok_0() == MinAddress(
        s,
        top,
        walk_top_pre,
    )));

    failure_conditions && success_conditions
}