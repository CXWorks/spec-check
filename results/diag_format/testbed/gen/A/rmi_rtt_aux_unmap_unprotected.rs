pub open spec fn rmi_rtt_aux_unmap_unprotected_spec(rd: Address, ipa: Address, index: UInt64, result: Result<(), RmiStatusCode>, top: Address, old_s: S, new_s: S) -> bool {
    let realm = RealmAt(old_s, rd);
    let new_realm = RealmAt(new_s, rd);
    let walk = RttWalk(new_s, new_realm, ipa, new_realm.rtt_level_start as int, index as int);
    let walk_top = RttSkipNonLiveEntries(new_s, RttAt(new_s, walk.rtt_addr), walk.level, ipa);
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(old_s, ipa, realm.rtt_level_start as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (((ipa as int) >= (pow2(realm.ipa_width as nat) as int) || AddrIsProtected(old_s, ipa, realm)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((realm.rtt_tree_per_plane == FEATURE_FALSE || (index as int) == RMM_RTT_TREE_PRIMARY || (index as int) > (realm.num_aux_planes as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && GranuleAt(old_s, rd).state == RD
        && AddrIsRttLevelAligned(old_s, ipa, realm.rtt_level_start as int)
        && (ipa as int) < (pow2(realm.ipa_width as nat) as int)
        && !AddrIsProtected(old_s, ipa, realm)
        && realm.rtt_tree_per_plane != FEATURE_FALSE
        && (index as int) != RMM_RTT_TREE_PRIMARY
        && (index as int) <= (realm.num_aux_planes as int))
        ==> (result.is_Ok()
            && walk.rtte.state == UNASSIGNED_NS
            && top == walk_top))
}
