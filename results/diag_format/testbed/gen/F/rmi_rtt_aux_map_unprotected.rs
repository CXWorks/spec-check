pub open spec fn rmi_rtt_aux_map_unprotected_spec(rd: Address, ipa: Address, index: UInt64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk_pri = RttWalk(old_s, realm, ipa, realm.rtt_level_start as int, RMM_RTT_TREE_PRIMARY as int);
    let walk_aux = RttWalk(old_s, realm, ipa, realm.rtt_level_start as int, index as int);
    let entry_idx = RttEntryIndex(old_s, ipa, walk_aux.level as int);
    let rtte_pri = RttEntryAt(old_s, walk_pri.rtt_addr, entry_idx as int);
    let rtte_aux = RttEntryAt(old_s, walk_aux.rtt_addr, entry_idx as int);

    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!GranuleAt(old_s, rd).state == RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(old_s, ipa, realm.rtt_level_start as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (ipa >= pow2(realm.ipa_width as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (AddrIsProtected(old_s, ipa, realm) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (ImplFeatures(old_s).rtt_tree_per_plane == RMM_FEATURE_FALSE ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (index == RMM_RTT_TREE_PRIMARY ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (index >= realm.num_aux_planes ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (walk_pri.valid ==> (walk_pri.level == walk_aux.level ==> ResultEqual(result, RMI_ERROR_RTT)))
    && (walk_pri.valid ==> (rtte_pri.state == RMM_RTT_ENTRY_STATE_UNASSIGNED_NS ==> ResultEqual(result, RMI_ERROR_RTT)))
    && (walk_aux.valid ==> (rtte_aux.state == rtte_pri.state))
    && (walk_aux.valid ==> (RmmRttMemAttrEqual(rtte_aux, rtte_pri, RMM_RTT_UNPROTECTED)))
    && (walk_aux.valid ==> (RmmRttS2APEqual(rtte_aux, rtte_pri, realm.rtt_s2ap_encoding)))
    && (walk_aux.valid ==> (rtte_aux.s2ap_direct.read == rtte_pri.s2ap_direct.read))
    && (walk_aux.valid ==> (rtte_aux.s2ap_direct.write == rtte_pri.s2ap_direct.write))
    && (walk_aux.valid ==> (rtte_aux.s2ap_indirect.base_index == rtte_pri.s2ap_indirect.base_index))
    && (walk_aux.valid ==> (rtte_aux.s2ap_indirect.overlay_index == rtte_pri.s2ap_indirect.overlay_index))
    && (walk_aux.valid ==> (rtte_aux.s2ap_direct.read || rtte_aux.s2ap_direct.write || rtte_aux.s2ap_indirect.base_index == RMM_RTT_S2AP_BASE_NO_ACCESS || rtte_aux.s2ap_indirect.overlay_index == 0))
    && (result.is_Ok() ==> (walk_aux.valid && walk_pri.valid && walk_aux.level == walk_pri.level && rtte_aux.state == rtte_pri.state && RmmRttMemAttrEqual(rtte_aux, rtte_pri, RMM_RTT_UNPROTECTED) && RmmRttS2APEqual(rtte_aux, rtte_pri, realm.rtt_s2ap_encoding) && rtte_aux.s2ap_direct.read == rtte_pri.s2ap_direct.read && rtte_aux.s2ap_direct.write == rtte_pri.s2ap_direct.write && rtte_aux.s2ap_indirect.base_index == rtte_pri.s2ap_indirect.base_index && rtte_aux.s2ap_indirect.overlay_index == rtte_pri.s2ap_indirect.overlay_index))
}