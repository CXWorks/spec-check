pub open spec fn rmi_rtt_aux_map_unprotected_spec(rd: Address, ipa: Address, index: UInt64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(old_s, ipa, RealmAt(old_s, rd).rtt_level_start as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (((ipa as int) >= (1int << (RealmAt(old_s, rd).ipa_width as int))
            || AddrIsProtected(old_s, ipa, RealmAt(old_s, rd))) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE
            || index == RMM_RTT_TREE_PRIMARY
            || (index as int) > (RealmAt(old_s, rd).num_aux_planes as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (RttWalk(old_s, RealmAt(old_s, rd), ipa, RealmAt(old_s, rd).rtt_level_start as int, RMM_RTT_TREE_PRIMARY as int).rtte.state == UNASSIGNED_NS
            ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(old_s, RealmAt(old_s, rd), ipa, RealmAt(old_s, rd).rtt_level_start as int, RMM_RTT_TREE_PRIMARY as int).level as int)))
    && ((AddrIsGranuleAligned(rd)
            && PaIsDelegable(old_s, rd)
            && GranuleAt(old_s, rd).state == RD
            && AddrIsRttLevelAligned(old_s, ipa, RealmAt(old_s, rd).rtt_level_start as int)
            && (ipa as int) < (1int << (RealmAt(old_s, rd).ipa_width as int))
            && !AddrIsProtected(old_s, ipa, RealmAt(old_s, rd))
            && RealmAt(old_s, rd).rtt_tree_per_plane != FEATURE_FALSE
            && index != RMM_RTT_TREE_PRIMARY
            && (index as int) <= (RealmAt(old_s, rd).num_aux_planes as int)
            && RttWalk(old_s, RealmAt(old_s, rd), ipa, RealmAt(old_s, rd).rtt_level_start as int, RMM_RTT_TREE_PRIMARY as int).rtte.state != UNASSIGNED_NS)
        ==> (result.is_Ok()
            && RttWalk(new_s, RealmAt(new_s, rd), ipa, RealmAt(new_s, rd).rtt_level_start as int, index as int).rtte.state
                == RttWalk(old_s, RealmAt(old_s, rd), ipa, RealmAt(old_s, rd).rtt_level_start as int, RMM_RTT_TREE_PRIMARY as int).rtte.state
            && RttMemAttrEqual(
                RttWalk(new_s, RealmAt(new_s, rd), ipa, RealmAt(new_s, rd).rtt_level_start as int, index as int).rtte,
                RttWalk(old_s, RealmAt(old_s, rd), ipa, RealmAt(old_s, rd).rtt_level_start as int, RMM_RTT_TREE_PRIMARY as int).rtte,
                RTT_UNPROTECTED)
            && RttS2APEqual(
                RttWalk(new_s, RealmAt(new_s, rd), ipa, RealmAt(new_s, rd).rtt_level_start as int, index as int).rtte,
                RttWalk(old_s, RealmAt(old_s, rd), ipa, RealmAt(old_s, rd).rtt_level_start as int, RMM_RTT_TREE_PRIMARY as int).rtte,
                RealmAt(old_s, rd).rtt_s2ap_encoding)
            && RttWalk(new_s, RealmAt(new_s, rd), ipa, RealmAt(new_s, rd).rtt_level_start as int, index as int).rtte.addr
                == RttWalk(old_s, RealmAt(old_s, rd), ipa, RealmAt(old_s, rd).rtt_level_start as int, RMM_RTT_TREE_PRIMARY as int).rtte.addr))
}