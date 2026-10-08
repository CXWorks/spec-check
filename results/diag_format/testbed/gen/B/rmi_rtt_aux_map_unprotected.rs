pub open spec fn rmi_rtt_aux_map_unprotected_spec(rd: Address, ipa: Address, index: UInt64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(old_s, ipa, RealmAt(old_s, rd).rtt_level_start as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (ipa >= pow2(RealmAt(old_s, rd).ipa_width as int) || AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > RealmAt(old_s, rd).num_aux_planes ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (RttWalk_(old_s, rd, ipa, RealmAt(old_s, rd).rtt_level_start as int).rtte.state == UNASSIGNED_NS ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(old_s, rd, ipa, RealmAt(old_s, rd).rtt_level_start as int).level)))
    && (result.is_Ok() ==> (RttWalk_(new_s, rd, ipa, RealmAt(new_s, rd).rtt_level_start as int).rtte.state == RttWalk_(old_s, rd, ipa, RealmAt(old_s, rd).rtt_level_start as int).rtte.state))
    && (result.is_Ok() ==> RttMemAttrEqual(RttWalk_(new_s, rd, ipa, RealmAt(new_s, rd).rtt_level_start as int).rtte, RttWalk_(old_s, rd, ipa, RealmAt(old_s, rd).rtt_level_start as int).rtte, RTT_UNPROTECTED))
    && (result.is_Ok() ==> RttS2APEqual(RttWalk_(new_s, rd, ipa, RealmAt(new_s, rd).rtt_level_start as int).rtte, RttWalk_(old_s, rd, ipa, RealmAt(old_s, rd).rtt_level_start as int).rtte, RealmAt(new_s, rd).rtt_s2ap_encoding))
    && (result.is_Ok() ==> RttWalk_(new_s, rd, ipa, RealmAt(new_s, rd).rtt_level_start as int).rtte.addr == RttWalk_(old_s, rd, ipa, RealmAt(old_s, rd).rtt_level_start as int).rtte.addr)
}