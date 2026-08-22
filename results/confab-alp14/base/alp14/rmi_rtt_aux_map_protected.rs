pub open spec fn rmi_rtt_aux_map_protected_spec(result: Result<(), RmiStatusCode>, state: RmiRttEntryState, ripas: RmiRipas, rd: Address, ipa: Address, index: u64, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE
            || index == RMM_RTT_TREE_PRIMARY
            || index > RealmAt(old_s, rd).num_aux_planes) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.state != ASSIGNED
            && RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.state != ASSIGNED_DEV
            && RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.state != ASSIGNED_VSMMU)
        ==> (ResultEqual(result, RMI_ERROR_RTT(RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).level as int))
            && state == RttEntryStateToRmi(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.state)
            && ripas == RipasToRmi(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.ripas)))
    && ((RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED
            && RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.ripas != RAM)
        ==> (ResultEqual(result, RMI_ERROR_RTT(RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).level as int))
            && state == RttEntryStateToRmi(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.state)
            && ripas == RipasToRmi(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.ripas)))
    && ((RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED_DEV
            && RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.ripas != DEV)
        ==> (ResultEqual(result, RMI_ERROR_RTT(RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).level as int))
            && state == RttEntryStateToRmi(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.state)
            && ripas == RipasToRmi(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.ripas)))
    && ((RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, index as int).rtte.state == AUX_DESTROYED)
        ==> (ResultEqual(result, RMI_ERROR_RTT_AUX(RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, index as int).level as int))
            && state == RttEntryStateToRmi(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, index as int).rtte.state)
            && ripas == RipasToRmi(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.ripas)))
    && ((RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, index as int).level < RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).level)
        ==> (ResultEqual(result, RMI_ERROR_RTT_AUX(RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, index as int).level as int))
            && state == RttEntryStateToRmi(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, index as int).rtte.state)
            && ripas == RipasToRmi(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.ripas)))
    && ((AddrIsGranuleAligned(old_s, rd)
            && PaIsDelegable(old_s, rd)
            && GranuleAt(old_s, rd).state == RD
            && AddrIsGranuleAligned(old_s, ipa)
            && AddrIsProtected(old_s, ipa, RealmAt(old_s, rd))
            && RealmAt(old_s, rd).rtt_tree_per_plane != FEATURE_FALSE
            && index != RMM_RTT_TREE_PRIMARY
            && index <= RealmAt(old_s, rd).num_aux_planes
            && (RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED
                || RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED_DEV
                || RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED_VSMMU)
            && !(RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED
                && RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.ripas != RAM)
            && !(RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED_DEV
                && RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.ripas != DEV)
            && RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, index as int).rtte.state != AUX_DESTROYED
            && RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, index as int).level >= RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).level)
        ==> (result.is_Ok()
            && RttWalk(new_s, RealmAt(new_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, index as int).rtte.state == ASSIGNED
            && RttWalk(new_s, RealmAt(new_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, index as int).rtte.attr_prot == RttWalk(new_s, RealmAt(new_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.attr_prot
            && RttWalk(new_s, RealmAt(new_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, index as int).rtte.sh == RttWalk(new_s, RealmAt(new_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.sh
            && RttWalk(new_s, RealmAt(new_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, index as int).rtte.addr == RttWalk(new_s, RealmAt(new_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.addr + (RttEntryIndex(new_s, ipa, RttWalk(new_s, RealmAt(new_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, index as int).level as int) * RttLevelSize(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, index as int).level as int))))
}