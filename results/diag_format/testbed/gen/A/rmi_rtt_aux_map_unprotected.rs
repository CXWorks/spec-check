pub open spec fn rmi_rtt_aux_map_unprotected_spec(rd: Address, ipa: Address, index: UInt64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk_pri = RttWalk(old_s, realm, ipa, realm.rtt_level_start as int, RMM_RTT_TREE_PRIMARY);
    let walk_aux = RttWalk(old_s, realm, ipa, realm.rtt_level_start as int, index as int);
    let entry_idx = RttEntryIndex(old_s, ipa, walk_aux.level);
    let rd_ok = AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && GranuleAt(old_s, rd).state == RD;
    let ipa_ok = AddrIsRttLevelAligned(old_s, ipa, realm.rtt_level_start as int)
        && (ipa as int) < (pow2(realm.ipa_width as nat) as int)
        && !AddrIsProtected(old_s, ipa, realm);
    let index_ok = realm.rtt_tree_per_plane == FEATURE_TRUE
        && (index as int) != RMM_RTT_TREE_PRIMARY
        && (index as int) <= (realm.num_aux_planes as int);
    let new_rtte = RttEntryAt(new_s, RttAt(new_s, walk_aux.rtt_addr), entry_idx);
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((rd_ok && !AddrIsRttLevelAligned(old_s, ipa, realm.rtt_level_start as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((rd_ok && ((ipa as int) >= (pow2(realm.ipa_width as nat) as int) || AddrIsProtected(old_s, ipa, realm))) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((rd_ok && (realm.rtt_tree_per_plane == FEATURE_FALSE
            || (index as int) == RMM_RTT_TREE_PRIMARY
            || (index as int) > (realm.num_aux_planes as int))) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((rd_ok && ipa_ok && index_ok && walk_pri.rtte.state == UNASSIGNED_NS) ==> ResultEqual(result, RMI_ERROR_RTT(walk_pri.level)))
    && (result.is_Err() ==> new_s == old_s)
    && ((rd_ok && ipa_ok && index_ok && walk_pri.rtte.state != UNASSIGNED_NS) ==> (
        result.is_Ok()
        && new_rtte.state == walk_pri.rtte.state
        && RttMemAttrEqual(new_rtte, walk_pri.rtte, RTT_UNPROTECTED)
        && RttS2APEqual(new_rtte, walk_pri.rtte, realm.rtt_s2ap_encoding)
        && new_rtte.addr == walk_pri.rtte.addr
    ))
}
