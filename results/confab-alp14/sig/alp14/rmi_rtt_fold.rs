pub open spec fn rmi_rtt_fold_spec(rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, rtt: Address, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((!RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int)
            || RttLevelIsStarting(old_s, RealmAt(old_s, rd), level as int))
        ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(old_s, ipa, level as int - 1) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((ipa as int) >= ((1u64 << RealmAt(old_s, rd).ipa_width) as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int - 1, RMM_RTT_TREE_PRIMARY).level < level as int - 1
        ==> ResultEqual(result, RMI_ERROR_RTT))
    && (RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int - 1, RMM_RTT_TREE_PRIMARY).rtte.state != TABLE
        ==> ResultEqual(result, RMI_ERROR_RTT))
    && (!RttIsHomogeneous(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int - 1, RMM_RTT_TREE_PRIMARY).rtte.addr))
        ==> ResultEqual(result, RMI_ERROR_RTT))
    && (AddrIsAuxRef(old_s, ipa, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_RTT))
    && ((AddrIsGranuleAligned(rd)
            && PaIsDelegable(old_s, rd)
            && GranuleAt(old_s, rd).state == RD
            && RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int)
            && !RttLevelIsStarting(old_s, RealmAt(old_s, rd), level as int)
            && AddrIsRttLevelAligned(old_s, ipa, level as int - 1)
            && (ipa as int) < ((1u64 << RealmAt(old_s, rd).ipa_width) as int)
            && RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int - 1, RMM_RTT_TREE_PRIMARY).level >= level as int - 1
            && RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int - 1, RMM_RTT_TREE_PRIMARY).rtte.state == TABLE
            && RttIsHomogeneous(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int - 1, RMM_RTT_TREE_PRIMARY).rtte.addr))
            && !AddrIsAuxRef(old_s, ipa, RealmAt(old_s, rd)))
        ==> (result.is_Ok()
            && RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int - 1, RMM_RTT_TREE_PRIMARY).rtte.state
                == RttFold(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int - 1, RMM_RTT_TREE_PRIMARY).rtte.addr)).state
            && ((RttFold(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int - 1, RMM_RTT_TREE_PRIMARY).rtte.addr)).state != UNASSIGNED
                    && RttFold(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int - 1, RMM_RTT_TREE_PRIMARY).rtte.addr)).state != UNASSIGNED_NS)
                ==> RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int - 1, RMM_RTT_TREE_PRIMARY).rtte.addr
                    == RttFold(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int - 1, RMM_RTT_TREE_PRIMARY).rtte.addr)).addr)
            && (RttFold(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int - 1, RMM_RTT_TREE_PRIMARY).rtte.addr)).state == ASSIGNED
                ==> (RttMemAttrEqual(
                        RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int - 1, RMM_RTT_TREE_PRIMARY).rtte,
                        RttFold(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int - 1, RMM_RTT_TREE_PRIMARY).rtte.addr)),
                        RTT_PROTECTED)
                    && RttS2APEqual(
                        RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int - 1, RMM_RTT_TREE_PRIMARY).rtte,
                        RttFold(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int - 1, RMM_RTT_TREE_PRIMARY).rtte.addr)),
                        S2AP_INDIRECT)))
            && (RttFold(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int - 1, RMM_RTT_TREE_PRIMARY).rtte.addr)).state == ASSIGNED_NS
                ==> (RttMemAttrEqual(
                        RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int - 1, RMM_RTT_TREE_PRIMARY).rtte,
                        RttFold(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int - 1, RMM_RTT_TREE_PRIMARY).rtte.addr)),
                        RTT_UNPROTECTED)
                    && RttS2APEqual(
                        RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int - 1, RMM_RTT_TREE_PRIMARY).rtte,
                        RttFold(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int - 1, RMM_RTT_TREE_PRIMARY).rtte.addr)),
                        RealmAt(new_s, rd).rtt_s2ap_encoding)))
            && (AddrIsProtected(old_s, ipa, RealmAt(old_s, rd))
                ==> RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int - 1, RMM_RTT_TREE_PRIMARY).rtte.ripas
                    == RttFold(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int - 1, RMM_RTT_TREE_PRIMARY).rtte.addr)).ripas)
            && GranuleAt(new_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int - 1, RMM_RTT_TREE_PRIMARY).rtte.addr).state == DELEGATED
            && rtt == RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int - 1, RMM_RTT_TREE_PRIMARY).rtte.addr))
}