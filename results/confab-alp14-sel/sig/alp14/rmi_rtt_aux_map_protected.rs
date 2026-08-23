pub open spec fn rmi_rtt_aux_map_protected_spec(rd: Address, ipa: Address, index: UInt64, result: Result<(), RmiStatusCode>, state: RmiRttEntryState, ripas: RmiRipas, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE
            || (index as int) == RMM_RTT_TREE_PRIMARY
            || (index as int) > RealmAt(old_s, rd).num_aux_planes as int)
        ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.state != ASSIGNED
            && RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.state != ASSIGNED_DEV
            && RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.state != ASSIGNED_VSMMU)
        ==> (ResultEqual(result, RMI_ERROR_RTT(RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).level))
            && state == RttEntryStateToRmi(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.state)
            && ripas == RipasToRmi(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.ripas)))
    && ((RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.state == ASSIGNED
            && RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.ripas != RAM)
        ==> (ResultEqual(result, RMI_ERROR_RTT(RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).level))
            && state == RttEntryStateToRmi(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.state)
            && ripas == RipasToRmi(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.ripas)))
    && ((RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.state == ASSIGNED_DEV
            && RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.ripas != DEV)
        ==> (ResultEqual(result, RMI_ERROR_RTT(RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).level))
            && state == RttEntryStateToRmi(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.state)
            && ripas == RipasToRmi(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.ripas)))
    && (RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, index as int).rtte.state == AUX_DESTROYED
        ==> (ResultEqual(result, RMI_ERROR_RTT_AUX(RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, index as int).level))
            && state == RttEntryStateToRmi(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, index as int).rtte.state)
            && ripas == RipasToRmi(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.ripas)))
    && (RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, index as int).level < RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).level
        ==> (ResultEqual(result, RMI_ERROR_RTT_AUX(RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, index as int).level))
            && state == RttEntryStateToRmi(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, index as int).rtte.state)
            && ripas == RipasToRmi(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.ripas)))
    && ((AddrIsGranuleAligned(old_s, rd)
            && PaIsDelegable(old_s, rd)
            && GranuleAt(old_s, rd).state == RD
            && AddrIsGranuleAligned(old_s, ipa)
            && AddrIsProtected(old_s, ipa, RealmAt(old_s, rd))
            && RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_TRUE
            && (index as int) != RMM_RTT_TREE_PRIMARY
            && (index as int) <= RealmAt(old_s, rd).num_aux_planes as int
            && (RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.state == ASSIGNED
                || RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.state == ASSIGNED_DEV
                || RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.state == ASSIGNED_VSMMU)
            && !(RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.state == ASSIGNED
                && RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.ripas != RAM)
            && !(RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.state == ASSIGNED_DEV
                && RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.ripas != DEV)
            && RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, index as int).rtte.state != AUX_DESTROYED
            && RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, index as int).level >= RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).level)
        ==> (result.is_Ok()
            && RttWalk(new_s, RealmAt(new_s, rd), ipa, RMM_RTT_PAGE_LEVEL, index as int).rtte.state == ASSIGNED
            && RttWalk(new_s, RealmAt(new_s, rd), ipa, RMM_RTT_PAGE_LEVEL, index as int).rtte.attr_prot == RttWalk(new_s, RealmAt(new_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.attr_prot
            && RttWalk(new_s, RealmAt(new_s, rd), ipa, RMM_RTT_PAGE_LEVEL, index as int).rtte.sh == RttWalk(new_s, RealmAt(new_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.sh
            && (RttWalk(new_s, RealmAt(new_s, rd), ipa, RMM_RTT_PAGE_LEVEL, index as int).rtte.addr as int)
                == (RttWalk(new_s, RealmAt(new_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.addr as int)
                    + RttEntryIndex(new_s, ipa, RttWalk(new_s, RealmAt(new_s, rd), ipa, RMM_RTT_PAGE_LEVEL, index as int).level)
                        * RttLevelSize(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa, RMM_RTT_PAGE_LEVEL, index as int).level)))
}