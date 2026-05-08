```verus
pub open spec fn rmi_rtt_aux_map_protected_spec(
    result: RmiCommandReturnCode,
    state: RmiRttEntryState,
    ripas: RmiRipas,
    old_s: S,
    new_s: S,
    rd: Address,
    ipa: Address,
    index: u64
) -> bool {
    let realm = RealmAt(rd);
    let walk_pri = RttWalk(old_s, realm, ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let walk_aux = RttWalk(old_s, realm, ipa, RMM_RTT_PAGE_LEVEL, index);
    let entry_idx = RttEntryIndex(old_s, ipa, walk_aux.level);

    // Failure conditions
    let rd_align_fail = !AddrIsGranuleAligned(rd) && ResultEqual(result, RMI_ERROR_INPUT);
    let rd_bound_fail = !PaIsDelegable(rd) && ResultEqual(result, RMI_ERROR_INPUT);
    let rd_state_fail = GranuleAt(rd).state != RD && ResultEqual(result, RMI_ERROR_INPUT);
    let ipa_align_fail = !AddrIsGranuleAligned(ipa) && ResultEqual(result, RMI_ERROR_INPUT);
    let ipa_bound_fail = !AddrIsProtected(ipa, realm) && ResultEqual(result, RMI_ERROR_INPUT);
    let index_bound_fail = (realm.rtt_tree_per_plane == FEATURE_FALSE
                               || index == RMM_RTT_TREE_PRIMARY
                               || index > realm.num_aux_planes)
                           && ResultEqual(result, RMI_ERROR_INPUT);

    let pri_unassigned_fail = (walk_pri.rtte.state != ASSIGNED
                                  && walk_pri.rtte.state != ASSIGNED_DEV
                                  && walk_pri.rtte.state != ASSIGNED_VSMMU)
                              && ResultEqual(result, RMI_ERROR_RTT)
                              && state == RttEntryStateToRmi(old_s, walk_pri.rtte.state)
                              && ripas == RipasToRmi(old_s, walk_pri.rtte.ripas);

    let pri_ram_fail = (walk_pri.rtte.state == ASSIGNED
                           && walk_pri.rtte.ripas != RAM)
                       && ResultEqual(result, RMI_ERROR_RTT)
                       && state == RttEntryStateToRmi(old_s, walk_pri.rtte.state)
                       && ripas == RipasToRmi(old_s, walk_pri.rtte.ripas);

    let pri_dev_fail = (walk_pri.rtte.state == ASSIGNED_DEV
                           && walk_pri.rtte.ripas != DEV)
                       && ResultEqual(result, RMI_ERROR_RTT)
                       && state == RttEntryStateToRmi(old_s, walk_pri.rtte.state)
                       && ripas == RipasToRmi(old_s, walk_pri.rtte.ripas);

    let aux_destroyed_fail = walk_aux.rtte.state == AUX_DESTROYED
                            && ResultEqual(result, RMI_ERROR_RTT_AUX)
                            && state == RttEntryStateToRmi(old_s, walk_aux.rtte.state)
                            && ripas == RipasToRmi(old_s, walk_pri.rtte.ripas);

    let level_fail = walk_aux.level < walk_pri.level
                    && ResultEqual(result, RMI_ERROR_RTT_AUX)
                    && state == RttEntryStateToRmi(old_s, walk_aux.rtte.state)
                    && ripas == RipasToRmi(old_s, walk_pri.rtte.ripas);

    // Success conditions
    let success = result == RMI_SUCCESS
                && walk_aux.rtte.state == ASSIGNED
                && walk_aux.rtte.attr_prot == walk_pri.rtte.attr_prot
                && walk_aux.rtte.sh == walk_pri.rtte.sh
                && walk_aux.rtte.addr == walk_pri.rtte.addr + (entry_idx * RttLevelSize(old_s, walk_aux.level));

    // Precondition checks imply failure conditions (in priority order)
    (!AddrIsGranuleAligned(rd) ==> rd_align_fail)
    && (!PaIsDelegable(rd) ==> rd_bound_fail)
    && (GranuleAt(rd).state != RD ==> rd_state_fail)
    && (!AddrIsGranuleAligned(ipa) ==> ipa_align_fail)
    && (!AddrIsProtected(ipa, realm) ==> ipa_bound_fail)
    && ((realm.rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > realm.num_aux_planes)
        ==> index_bound_fail)
    && ((walk_pri.rtte.state != ASSIGNED && walk_pri.rtte.state != ASSIGNED_DEV && walk_pri.rtte.state != ASSIGNED_VSMMU)
        ==> pri_unassigned_fail)
    && ((walk_pri.rtte.state == ASSIGNED && walk_pri.rtte.ripas != RAM)
        ==> pri_ram_fail)
    && ((walk_pri.rtte.state == ASSIGNED_DEV && walk_pri.rtte.ripas != DEV)
        ==> pri_dev_fail)
    && (walk_aux.rtte.state == AUX_DESTROYED ==> aux_destroyed_fail)
    && (walk_aux.level < walk_pri.level ==> level_fail)
    // All preconditions satisfied implies success
    && (AddrIsGranuleAligned(rd)
        && PaIsDelegable(rd)
        && GranuleAt(rd).state == RD
        && AddrIsGranuleAligned(ipa)
        && AddrIsProtected(ipa, realm)
        && (realm.rtt_tree_per_plane != FEATURE_FALSE
            && index != RMM_RTT_TREE_PRIMARY
            && index <= realm.num_aux_planes)
        && (walk_pri.rtte.state == ASSIGNED || walk_pri.rtte.state == ASSIGNED_DEV || walk_pri.rtte.state == ASSIGNED_VSMMU)
        && !(walk_pri.rtte.state == ASSIGNED && walk_pri.rtte.ripas != RAM)
        && !(walk_pri.rtte.state == ASSIGNED_DEV && walk_pri.rtte.ripas != DEV)
        && walk_aux.rtte.state != AUX_DESTROYED
        && walk_aux.level >= walk_pri.level
        ==> success)
}
```