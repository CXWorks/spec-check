pub open spec fn rmi_rtt_aux_unmap_unprotected_spec(rd: Address, ipa: Address, index: UInt64, result: Result<(), RmiStatusCode>, top: Address, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(old_s, ipa, RealmAt(old_s, rd).rtt_level_start as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (((ipa as int) >= (1int << (RealmAt(old_s, rd).ipa_width as int))
            || AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)))
        ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE
            || index == RMM_RTT_TREE_PRIMARY
            || (index as int) > (RealmAt(old_s, rd).num_aux_planes as int))
        ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((AddrIsGranuleAligned(old_s, rd)
            && PaIsDelegable(old_s, rd)
            && GranuleAt(old_s, rd).state == RD
            && AddrIsRttLevelAligned(old_s, ipa, RealmAt(old_s, rd).rtt_level_start as int)
            && (ipa as int) < (1int << (RealmAt(old_s, rd).ipa_width as int))
            && !AddrIsProtected(old_s, ipa, RealmAt(old_s, rd))
            && RealmAt(old_s, rd).rtt_tree_per_plane != FEATURE_FALSE
            && index != RMM_RTT_TREE_PRIMARY
            && (index as int) <= (RealmAt(old_s, rd).num_aux_planes as int))
        ==> (result.is_Ok()
            && RttWalk(new_s, RealmAt(new_s, rd), ipa, RealmAt(new_s, rd).rtt_level_start as int, index as int).rtte.state == UNASSIGNED_NS
            && top == RttSkipNonLiveEntries(
                    new_s,
                    RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa, RealmAt(new_s, rd).rtt_level_start as int, index as int).rtt_addr),
                    RttWalk(new_s, RealmAt(new_s, rd), ipa, RealmAt(new_s, rd).rtt_level_start as int, index as int).level as int,
                    ipa)))
}