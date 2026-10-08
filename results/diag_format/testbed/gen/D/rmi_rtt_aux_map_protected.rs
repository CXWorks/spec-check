pub open spec fn rmi_rtt_aux_map_protected_spec(rd: Address, ipa: Address, index: UInt64, result: Result<(), RmiStatusCode>, state: RmiRttEntryState, ripas: RmiRipas, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (RealmAt(old_s, rd).rtt_tree_per_plane == RMM_FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > RealmAt(old_s, rd).num_aux_planes ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (RttWalk_(old_s, rd, ipa, RMM_RTT_PAGE_LEVEL as int).state != ASSIGNED && RttWalk_(old_s, rd, ipa, RMM_RTT_PAGE_LEVEL as int).state != ASSIGNED_DEV && RttWalk_(old_s, rd, ipa, RMM_RTT_PAGE_LEVEL as int).state != ASSIGNED_VSMMU ==> ResultEqual(result, RMI_ERROR_RTT))
    && (RttWalk_(old_s, rd, ipa, RMM_RTT_PAGE_LEVEL as int).state == ASSIGNED && RttWalk_(old_s, rd, ipa, RMM_RTT_PAGE_LEVEL as int).rtte.ripas != RAM ==> ResultEqual(result, RMI_ERROR_RTT))
    && (RttWalk_(old_s, rd, ipa, RMM_RTT_PAGE_LEVEL as int).state == ASSIGNED_DEV && RttWalk_(old_s, rd, ipa, RMM_RTT_PAGE_LEVEL as int).rtte.ripas != DEV ==> ResultEqual(result, RMI_ERROR_RTT))
    && (RttWalk_(old_s, rd, ipa, index as int).state == AUX_DESTROYED ==> ResultEqual(result, RMI_ERROR_RTT_AUX))
    && (RttWalk_(old_s, rd, ipa, index as int).level < RttWalk_(old_s, rd, ipa, RMM_RTT_PAGE_LEVEL as int).level ==> ResultEqual(result, RMI_ERROR_RTT_AUX))
    && (result.is_Ok() ==> (RttWalk_(old_s, rd, ipa, index as int).state == ASSIGNED && RttWalk_(old_s, rd, ipa, index as int).rtte.attr_prot == RttWalk_(old_s, rd, ipa, RMM_RTT_PAGE_LEVEL as int).rtte.attr_prot && RttWalk_(old_s, rd, ipa, index as int).rtte.sh == RttWalk_(old_s, rd, ipa, RMM_RTT_PAGE_LEVEL as int).rtte.sh && RttWalk_(old_s, rd, ipa, index as int).rtte.addr == RttWalk_(old_s, rd, ipa, RMM_RTT_PAGE_LEVEL as int).rtte.addr + (RttEntryIndex(old_s, ipa, RttWalk_(old_s, rd, ipa, index as int).level) as int) * pow2(RttWalk_(old_s, rd, ipa, index as int).level as int)))
}