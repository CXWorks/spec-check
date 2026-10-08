pub open spec fn rmi_rtt_aux_map_unprotected_spec(rd: Address, ipa: Address, index: UInt64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk_pri = RttWalk(old_s, realm, ipa, realm.rtt_level_start as int);
    let walk_aux = RttWalk(old_s, realm, ipa, realm.rtt_level_start as int, index as int);
    let entry_idx = RttEntryIndex(ipa, walk_aux.level as int);

    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsAligned(old_s, ipa, pow2(realm.ipa_width as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (ipa >= pow2(realm.ipa_width as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (AddrIsProtected(old_s, ipa, realm) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (realm.rtt_tree_per_plane == RMM_FEATURE_FALSE ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (index == RMM_RTT_TREE_PRIMARY ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (index > realm.num_aux_planes ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (walk_pri.rtte.state == RMM_RTT_ENTRY_UNASSIGNED_NS ==> ResultEqual(result, RMI_ERROR_RTT))
    && (result.is_Ok() ==> (walk_aux.rtte.state == walk_pri.rtte.state))
    && (result.is_Ok() ==> RmmRttMemAttrEqual(walk_aux.rtte, walk_pri.rtte, RMM_RTT_PROTECTED))
    && (result.is_Ok() ==> RmmRttS2APEqual(walk_aux.rtte, walk_pri.rtte, realm.rtt_s2ap_encoding))
    && (result.is_Ok() ==> (walk_aux.rtte.s2ap_direct.read == walk_pri.rtte.s2ap_direct.read && walk_aux.rtte.s2ap_direct.write == walk_pri.rtte.s2ap_direct.write))
}