pub open spec fn rmi_rtt_fold_spec(
    result: RmiCommandReturnCode,
    rtt: Address,
    old_s: S,
    new_s: S,
) -> bool {
    let rd = old_s.cmd_input.rd;
    let ipa = old_s.cmd_input.ipa;
    let level = old_s.cmd_input.level;

    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, level - 1, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let fold_pre = RttFold(old_s, RttAt(old_s, walk.rtte.addr));

    // Failure conditions
    let rd_align_fail = !AddrIsGranuleAligned(rd);
    let rd_bound_fail = !PaIsDelegable(rd);
    let rd_state_fail = GranuleAt(old_s, rd).state != RD;
    let level_bound_fail = !RttLevelIsValid(old_s, realm, level) || RttLevelIsStarting(
        old_s,
        realm,
        level,
    );
    let ipa_align_fail = !AddrIsRttLevelAligned(ipa, level - 1);
    let ipa_bound_fail = UInt(ipa) >= (1 << realm.ipa_width);
    let rtt_walk_fail = walk.level < level - 1;
    let rtte_state_fail = walk.rtte.state != TABLE;
    let rtt_homo_fail = !RttIsHomogeneous(old_s, RttAt(old_s, walk.rtte.addr));
    let aux_ref_fail = AddrIsAuxRef(ipa, realm);

    // Input validation failures
    (rd_align_fail ==> result == RMI_ERROR_INPUT) && (rd_bound_fail ==> result == RMI_ERROR_INPUT)
        && (rd_state_fail ==> result == RMI_ERROR_INPUT) && (level_bound_fail ==> result
        == RMI_ERROR_INPUT) && (ipa_align_fail ==> result == RMI_ERROR_INPUT) && (ipa_bound_fail
        ==> result == RMI_ERROR_INPUT) &&
    // RTT walk and table failures
    (rtt_walk_fail ==> result.is_error() && result.error_code() == RMI_ERROR_RTT
        && result.error_index() == walk.level) && (rtte_state_fail ==> result.is_error()
        && result.error_code() == RMI_ERROR_RTT && result.error_index() == walk.level) && (
    rtt_homo_fail ==> result.is_error() && result.error_code() == RMI_ERROR_RTT
        && result.error_index() == level) && (aux_ref_fail ==> result.is_error()
        && result.error_code() == RMI_ERROR_RTT && result.error_index() == walk.level)
        &&
    // Success conditions (when no failures occur)
    ((!rd_align_fail && !rd_bound_fail && !rd_state_fail && !level_bound_fail && !ipa_align_fail
        && !ipa_bound_fail && !rtt_walk_fail && !rtte_state_fail && !rtt_homo_fail && !aux_ref_fail)
        ==> (result == RMI_SUCCESS && rtt == walk.rtte.addr
        &&
    // RTTE state matches folded entry
    RttEntryAt(new_s, RttAt(new_s, walk.rtte.addr), entry_idx).state == fold_pre.state
        &&
    // RTTE address matches (if not unassigned)
    ((fold_pre.state != UNASSIGNED && fold_pre.state != UNASSIGNED_NS) ==> RttEntryAt(
        new_s,
        RttAt(new_s, walk.rtte.addr),
        entry_idx,
    ).addr == fold_pre.addr)
        &&
    // Memory attributes preserved for protected
    ((fold_pre.state == ASSIGNED) ==> (RttMemAttrEqual(
        RttEntryAt(new_s, RttAt(new_s, walk.rtte.addr), entry_idx),
        fold_pre,
        RTT_PROTECTED,
    ) && RttS2APEqual(
        RttEntryAt(new_s, RttAt(new_s, walk.rtte.addr), entry_idx),
        fold_pre,
        S2AP_INDIRECT,
    ))) &&
    // Memory attributes preserved for unprotected
    ((fold_pre.state == ASSIGNED_NS) ==> (RttMemAttrEqual(
        RttEntryAt(new_s, RttAt(new_s, walk.rtte.addr), entry_idx),
        fold_pre,
        RTT_UNPROTECTED,
    ) && RttS2APEqual(
        RttEntryAt(new_s, RttAt(new_s, walk.rtte.addr), entry_idx),
        fold_pre,
        realm.rtt_s2ap_encoding,
    ))) &&
    // RIPAS preserved for protected addresses
    ((AddrIsProtected(ipa, realm)) ==> RttEntryAt(
        new_s,
        RttAt(new_s, walk.rtte.addr),
        entry_idx,
    ).ripas == fold_pre.ripas) &&
    // RTT granule becomes delegated
    GranuleAt(new_s, walk.rtte.addr).state == DELEGATED))
}