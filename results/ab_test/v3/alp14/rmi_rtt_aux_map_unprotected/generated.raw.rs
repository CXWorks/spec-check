pub open spec fn rmi_rtt_aux_map_unprotected_spec(result: RmiCommandReturnCode, rd: Address, ipa: Address, index: u64, old_s: S, new_s: S) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk_pri = RttWalk(old_s, realm, ipa, realm.rtt_level_start, RMM_RTT_TREE_PRIMARY);
    let walk_aux = RttWalk(old_s, realm, ipa, realm.rtt_level_start, index);
    let entry_idx = RttEntryIndex(old_s, ipa, walk_aux.level);
    
    (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(ipa, realm.rtt_level_start) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (((ipa as int) >= (1 << realm.ipa_width) || AddrIsProtected(ipa, realm)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((realm.rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > realm.num_aux_planes) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (walk_pri.rtte.state == UNASSIGNED_NS ==> ResultEqual(result, RMI_ERROR_RTT))
    && ((AddrIsGranuleAligned(rd) && PaIsDelegable(rd) && GranuleAt(old_s, rd).state == RD
         && AddrIsRttLevelAligned(ipa, realm.rtt_level_start)
         && !((ipa as int) >= (1 << realm.ipa_width) || AddrIsProtected(ipa, realm))
         && !(realm.rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > realm.num_aux_planes)
         && walk_pri.rtte.state != UNASSIGNED_NS)
        ==> (result.is_Ok()
            && RttEntryAt(new_s, RttAt(new_s, walk_aux.rtt_addr), entry_idx).state == walk_pri.rtte.state
            && RttMemAttrEqual(RttEntryAt(new_s, RttAt(new_s, walk_aux.rtt_addr), entry_idx), walk_pri.rtte, RTT_UNPROTECTED)
            && RttS2APEqual(RttEntryAt(new_s, RttAt(new_s, walk_aux.rtt_addr), entry_idx), walk_pri.rtte, realm.rtt_s2ap_encoding)
            && RttEntryAt(new_s, RttAt(new_s, walk_aux.rtt_addr), entry_idx).addr == walk_pri.rtte.addr))
}