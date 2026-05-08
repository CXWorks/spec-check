pub open spec fn RMI_RTT_AUX_FOLD_spec(
    old_s: S,
    new_s: S,
    rd: Address,
    ipa: Address,
    level: int,
    index: u64,
    result: RmiCommandReturnCode,
    rtt_out: Address,
) -> bool {
    let realm = RealmAt(rd);
    let walk = RttWalk(old_s, realm, ipa, level - 1, index as int);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let fold_pre = RttFold(old_s, RttAt(old_s, walk.rtte.addr));

    // Failure conditions
    let rd_align_fail = !AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let rd_bound_fail = !PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let rd_state_fail = GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT);
    let level_bound_fail = (!RttLevelIsValid(old_s, realm, level) || RttLevelIsStarting(
        old_s,
        realm,
        level,
    )) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let ipa_align_fail = !AddrIsRttLevelAligned(ipa, level - 1) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );
    let ipa_bound_fail = !AddrIsProtected(ipa, realm) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let index_bound_fail = (realm.rtt_tree_per_plane == FEATURE_FALSE || index
        == RMM_RTT_TREE_PRIMARY || index > realm.num_aux_planes as u64) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );
    let rtt_walk_fail = walk.level < level - 1 ==> result.is_Err() && result.get_Err_0()
        == RMI_ERROR_RTT_AUX;
    let rtte_state_fail = walk.rtte.state != TABLE ==> result.is_Err() && result.get_Err_0()
        == RMI_ERROR_RTT_AUX;
    let rtt_homo_fail = !RttIsHomogeneous(old_s, RttAt(old_s, walk.rtte.addr)) ==> result.is_Err()
        && result.get_Err_0() == RMI_ERROR_RTT_AUX;

    // Success conditions
    let rtte_state_succ = (walk.rtte.state == fold_pre.state);
    let rtte_addr_succ = ((fold_pre.state != UNASSIGNED && fold_pre.state != UNASSIGNED_NS)
        ==> walk.rtte.addr == fold_pre.addr);
    let rtte_attr_prot_succ = (fold_pre.state == ASSIGNED ==> (RttMemAttrEqual(
        walk.rtte,
        fold_pre,
        RTT_PROTECTED,
    ) && RttS2APEqual(walk.rtte, fold_pre, S2AP_INDIRECT)));
    let rtte_attr_unprot_succ = (fold_pre.state == ASSIGNED_NS ==> (RttMemAttrEqual(
        walk.rtte,
        fold_pre,
        RTT_UNPROTECTED,
    ) && RttS2APEqual(walk.rtte, fold_pre, realm.rtt_s2ap_encoding)));
    let rtte_ripas_succ = (AddrIsProtected(ipa, realm) ==> walk.rtte.ripas == fold_pre.ripas);
    let rtt_state_succ = GranuleAt(new_s, walk.rtte.addr).state == DELEGATED;
    let rtt_output_succ = (rtt_out == walk.rtte.addr);

    // All conditions combined
    rd_align_fail && rd_bound_fail && rd_state_fail && level_bound_fail && ipa_align_fail
        && ipa_bound_fail && index_bound_fail && rtt_walk_fail && rtte_state_fail && rtt_homo_fail
        && rtte_state_succ && rtte_addr_succ && rtte_attr_prot_succ && rtte_attr_unprot_succ
        && rtte_ripas_succ && rtt_state_succ && rtt_output_succ
}