pub open spec fn rmi_rtt_aux_map_protected_spec(rd: Address, ipa: Address, index: UInt64, result: Result<(), RmiStatusCode>, state: RmiRttEntryState, ripas: RmiRipas, old_s: S, new_s: S) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk_pri = RttWalk_(old_s, rd, ipa, RMM_RTT_PAGE_LEVEL as int);
    let walk_aux = RttWalk_(old_s, rd, ipa, RMM_RTT_PAGE_LEVEL as int);
    let entry_idx = RttEntryIndex(old_s, ipa, walk_aux.level as int);
    let entry_pri = RttEntryAt(old_s, RttWalk_(old_s, realm, ipa, RMM_RTT_PAGE_LEVEL as int).rtt_addr, entry_idx as int);
    let entry_aux = RttEntryAt(old_s, RttWalk_(old_s, realm, ipa, RMM_RTT_PAGE_LEVEL as int).rtt_addr, entry_idx as int);
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!GranuleAt(old_s, rd).state == RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsProtected(old_s, ipa, realm) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (ImplFeatures(old_s).rtt_tree_per_plane == RMM_FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > ImplFeatures(old_s).num_aux_planes ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (walk_pri.level != RMM_RTT_PAGE_LEVEL || walk_pri.valid == RMM_FALSE ==> ResultEqual(result, RMI_ERROR_RTT))
    && (walk_pri.level != RMM_RTT_PAGE_LEVEL || walk_pri.valid == RMM_FALSE ==> ResultEqual(result, RMI_ERROR_RTT))
    && (walk_aux.level != RMM_RTT_PAGE_LEVEL || walk_aux.valid == RMM_FALSE ==> ResultEqual(result, RMI_ERROR_RTT_AUX))
    && (walk_aux.level != RMM_RTT_PAGE_LEVEL || walk_aux.valid == RMM_FALSE ==> ResultEqual(result, RMI_ERROR_RTT_AUX))
    && (ResultEqual(result, RMI_ERROR_RTT) ==> (entry_pri.state == ASSIGNED || entry_pri.state == ASSIGNED_DEV || entry_pri.state == ASSIGNED_VSMMU))
    && (ResultEqual(result, RMI_ERROR_RTT) && entry_pri.state == ASSIGNED ==> RipasToRmi(old_s, entry_pri.ripas) != RMI_RAM)
    && (ResultEqual(result, RMI_ERROR_RTT) && entry_pri.state == ASSIGNED_DEV ==> RipasToRmi(old_s, entry_pri.ripas) != RMI_DEV)
    && (ResultEqual(result, RMI_ERROR_RTT_AUX) ==> entry_aux.state == AUX_DESTROYED)
    && (ResultEqual(result, RMI_ERROR_RTT_AUX) ==> walk_aux.level < walk_pri.level)
    && (result.is_Ok() ==> entry_aux.state == ASSIGNED)
    && (result.is_Ok() ==> entry_aux.attr_prot == entry_pri.attr_prot)
    && (result.is_Ok() ==> entry_aux.sh == entry_pri.sh)
}