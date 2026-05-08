pub open spec fn RMI_RTT_AUX_MAP_PROTECTED_spec(
    old_s: S,
    new_s: S,
    rd: Address,
    ipa: Address,
    index: u64,
    result: Result<(), RmiStatusCode>,
    state: RmiRttEntryState,
    ripas: RmiRipas
) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk_pri = RttWalk(old_s, realm, ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let walk_aux = RttWalk(old_s, realm, ipa, RMM_RTT_PAGE_LEVEL, index);
    let entry_idx = RttEntryIndex(old_s, ipa, walk_aux.level);
    
    // Failure conditions with proper ordering
    (
        // rd_align check
        !AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT)
    ) && (
        // rd_bound and rd_state checks (ordered before RTT checks)
        (!PaIsDelegable(rd) || GranuleAt(old_s, rd).state != RD) ==> ResultEqual(result, RMI_ERROR_INPUT)
    ) && (
        // ipa_align check
        !AddrIsGranuleAligned(ipa) ==> ResultEqual(result, RMI_ERROR_INPUT)
    ) && (
        // ipa_bound and index_bound checks (ordered before RTT checks)
        (!AddrIsProtected(ipa, realm) || 
         realm.rtt_tree_per_plane == FEATURE_FALSE ||
         index == RMM_RTT_TREE_PRIMARY ||
         index > realm.num_aux_planes) ==> ResultEqual(result, RMI_ERROR_INPUT)
    ) && (
        // pri_unassigned check
        (walk_pri.rtte.state != ASSIGNED &&
         walk_pri.rtte.state != ASSIGNED_DEV &&
         walk_pri.rtte.state != ASSIGNED_VSMMU) ==>
        (ResultEqual(result, RMI_ERROR_RTT) &&
         state == RttEntryStateToRmi(old_s, walk_pri.rtte.state) &&
         ripas == RipasToRmi(old_s, walk_pri.rtte.ripas))
    ) && (
        // pri_ram check
        (walk_pri.rtte.state == ASSIGNED && walk_pri.rtte.ripas != RAM) ==>
        (ResultEqual(result, RMI_ERROR_RTT) &&
         state == RttEntryStateToRmi(old_s, walk_pri.rtte.state) &&
         ripas == RipasToRmi(old_s, walk_pri.rtte.ripas))
    ) && (
        // pri_dev check
        (walk_pri.rtte.state == ASSIGNED_DEV && walk_pri.rtte.ripas != DEV) ==>
        (ResultEqual(result, RMI_ERROR_RTT) &&
         state == RttEntryStateToRmi(old_s, walk_pri.rtte.state) &&
         ripas == RipasToRmi(old_s, walk_pri.rtte.ripas))
    ) && (
        // aux_destroyed check
        walk_aux.rtte.state == AUX_DESTROYED ==>
        (ResultEqual(result, RMI_ERROR_RTT_AUX) &&
         state == RttEntryStateToRmi(old_s, walk_aux.rtte.state) &&
         ripas == RipasToRmi(old_s, walk_pri.rtte.ripas))
    ) && (
        // level check
        walk_aux.level < walk_pri.level ==>
        (ResultEqual(result, RMI_ERROR_RTT_AUX) &&
         state == RttEntryStateToRmi(old_s, walk_aux.rtte.state) &&
         ripas == RipasToRmi(old_s, walk_pri.rtte.ripas))
    ) && (
        // Success conditions
        (AddrIsGranuleAligned(rd) &&
         PaIsDelegable(rd) &&
         GranuleAt(old_s, rd).state == RD &&
         AddrIsGranuleAligned(ipa) &&
         AddrIsProtected(ipa, realm) &&
         !(realm.rtt_tree_per_plane == FEATURE_FALSE ||
           index == RMM_RTT_TREE_PRIMARY ||
           index > realm.num_aux_planes) &&
         (walk_pri.rtte.state == ASSIGNED ||
          walk_pri.rtte.state == ASSIGNED_DEV ||
          walk_pri.rtte.state == ASSIGNED_VSMMU) &&
         !(walk_pri.rtte.state == ASSIGNED && walk_pri.rtte.ripas != RAM) &&
         !(walk_pri.rtte.state == ASSIGNED_DEV && walk_pri.rtte.ripas != DEV) &&
         walk_aux.rtte.state != AUX_DESTROYED &&
         walk_aux.level >= walk_pri.level) ==>
        (result.is_Ok() &&
         RttEntryAt(new_s, RttAt(new_s, walk_aux.rtt_addr), entry_idx as int).state == ASSIGNED &&
         RttEntryAt(new_s, RttAt(new_s, walk_aux.rtt_addr), entry_idx as int).attr_prot == walk_pri.rtte.attr_prot &&
         RttEntryAt(new_s, RttAt(new_s, walk_aux.rtt_addr), entry_idx as int).sh == walk_pri.rtte.sh &&
         RttEntryAt(new_s, RttAt(new_s, walk_aux.rtt_addr), entry_idx as int).addr == 
             walk_pri.rtte.addr + (entry_idx * RttLevelSize(old_s, walk_aux.level)))
    )
}