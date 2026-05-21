pub open spec fn rmi_rtt_aux_map_protected_spec(
    result: RmiCommandReturnCode,
    state: u8,
    ripas: u8,
    old_s: S,
    new_s: S,
    rd: Address,
    ipa: Address,
    index: u64,
) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk_pri = RttWalk(old_s, realm, ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let walk_aux = RttWalk(old_s, realm, ipa, RMM_RTT_PAGE_LEVEL, index);
    let entry_idx = RttEntryIndex(old_s, ipa, walk_aux.level);

    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(
        old_s,
        rd,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(old_s, rd).state != RD
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsGranuleAligned(old_s, ipa)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsProtected(old_s, ipa, realm)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((realm.rtt_tree_per_plane == FEATURE_FALSE
        || index == RMM_RTT_TREE_PRIMARY || index > realm.num_aux_planes) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && ((walk_pri.rtte.state != ASSIGNED && walk_pri.rtte.state != ASSIGNED_DEV
        && walk_pri.rtte.state != ASSIGNED_VSMMU) ==> (ResultEqual(result, RMI_ERROR_RTT) && state
        == RttEntryStateToRmi(old_s, walk_pri.rtte.state) && ripas == RipasToRmi(
        old_s,
        walk_pri.rtte.ripas,
    ))) && ((walk_pri.rtte.state == ASSIGNED && walk_pri.rtte.ripas != RAM) ==> (ResultEqual(
        result,
        RMI_ERROR_RTT,
    ) && state == RttEntryStateToRmi(old_s, walk_pri.rtte.state) && ripas == RipasToRmi(
        old_s,
        walk_pri.rtte.ripas,
    ))) && ((walk_pri.rtte.state == ASSIGNED_DEV && walk_pri.rtte.ripas != DEV) ==> (ResultEqual(
        result,
        RMI_ERROR_RTT,
    ) && state == RttEntryStateToRmi(old_s, walk_pri.rtte.state) && ripas == RipasToRmi(
        old_s,
        walk_pri.rtte.ripas,
    ))) && (walk_aux.rtte.state == AUX_DESTROYED ==> (ResultEqual(result, RMI_ERROR_RTT_AUX)
        && state == RttEntryStateToRmi(old_s, walk_aux.rtte.state) && ripas == RipasToRmi(
        old_s,
        walk_pri.rtte.ripas,
    ))) && (walk_aux.level < walk_pri.level ==> (ResultEqual(result, RMI_ERROR_RTT_AUX) && state
        == RttEntryStateToRmi(old_s, walk_aux.rtte.state) && ripas == RipasToRmi(
        old_s,
        walk_pri.rtte.ripas,
    ))) && ((AddrIsGranuleAligned(old_s, rd) && PaIsDelegable(old_s, rd) && GranuleAt(
        old_s,
        rd,
    ).state == RD && AddrIsGranuleAligned(old_s, ipa) && AddrIsProtected(old_s, ipa, realm) && !(
    realm.rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index
        > realm.num_aux_planes) && (walk_pri.rtte.state == ASSIGNED || walk_pri.rtte.state
        == ASSIGNED_DEV || walk_pri.rtte.state == ASSIGNED_VSMMU) && !(walk_pri.rtte.state
        == ASSIGNED && walk_pri.rtte.ripas != RAM) && !(walk_pri.rtte.state == ASSIGNED_DEV
        && walk_pri.rtte.ripas != DEV) && walk_aux.rtte.state != AUX_DESTROYED && walk_aux.level
        >= walk_pri.level) ==> (result == RMI_SUCCESS && RttEntryAt(
        new_s,
        RttAt(new_s, walk_aux.rtte.addr),
        entry_idx,
    ).state == ASSIGNED && RttEntryAt(new_s, RttAt(new_s, walk_aux.rtte.addr), entry_idx).attr_prot
        == walk_pri.rtte.attr_prot && RttEntryAt(
        new_s,
        RttAt(new_s, walk_aux.rtte.addr),
        entry_idx,
    ).sh == walk_pri.rtte.sh && RttEntryAt(new_s, RttAt(new_s, walk_aux.rtte.addr), entry_idx).addr
        == walk_pri.rtte.addr + (entry_idx * RttLevelSize(old_s, walk_aux.level))))
}