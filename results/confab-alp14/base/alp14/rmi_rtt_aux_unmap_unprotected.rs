pub open spec fn rmi_rtt_aux_unmap_unprotected_spec(result: Result<(), RmiStatusCode>, rd: Address, ipa: Address, index: u64, top: Address, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(old_s, ipa, RealmAt(old_s, rd).rtt_level_start as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (((ipa as int) >= (1int << RealmAt(old_s, rd).ipa_width) || AddrIsProtected(old_s, ipa, RealmAt(old_s, rd))) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE
            || index == RMM_RTT_TREE_PRIMARY
            || index as int > RealmAt(old_s, rd).num_aux_planes as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((AddrIsGranuleAligned(old_s, rd)
            && PaIsDelegable(old_s, rd)
            && GranuleAt(old_s, rd).state == RD
            && AddrIsRttLevelAligned(old_s, ipa, RealmAt(old_s, rd).rtt_level_start as int)
            && (ipa as int) < (1int << RealmAt(old_s, rd).ipa_width)
            && !AddrIsProtected(old_s, ipa, RealmAt(old_s, rd))
            && RealmAt(old_s, rd).rtt_tree_per_plane != FEATURE_FALSE
            && index != RMM_RTT_TREE_PRIMARY
            && index as int <= RealmAt(old_s, rd).num_aux_planes as int)
        ==> result.is_Ok()
            && RttEntryAt(
                    new_s,
                    RttAt(new_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, RealmAt(old_s, rd).rtt_level_start as int, index as int).rtt_addr),
                    RttEntryIndex(old_s, ipa, RttWalk(old_s, RealmAt(old_s, rd), ipa, RealmAt(old_s, rd).rtt_level_start as int, index as int).level as int)
               ).state == UNASSIGNED_NS
            && top == RttSkipNonLiveEntries(
                    old_s,
                    RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, RealmAt(old_s, rd).rtt_level_start as int, index as int).rtt_addr),
                    RttWalk(old_s, RealmAt(old_s, rd), ipa, RealmAt(old_s, rd).rtt_level_start as int, index as int).level as int,
                    ipa
               ))
}