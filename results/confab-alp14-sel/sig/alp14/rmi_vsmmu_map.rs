pub open spec fn rmi_vsmmu_map_spec(rd: Address, vsmmu_ptr: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((!RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) || (level as int) < 2) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (RealmAt(old_s, rd).state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, vsmmu_ptr).state != VSMMU ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(old_s, ipa, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((ipa as int) < (VsmmuAt(old_s, vsmmu_ptr).reg_base as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).level < (level as int)
            ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).level)))
    && (RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).rtte.state != UNASSIGNED
            ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).level)))
    && (RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).rtte.ripas != EMPTY
            ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).level)))
    && ((ipa as int) + (RttLevelSize(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).level) - 1)
            >= (VsmmuAt(old_s, vsmmu_ptr).reg_top as int)
            ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).level)))
    && ((ImplFeatures(old_s).feat_da == FEATURE_TRUE
        && AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && GranuleAt(old_s, rd).state == RD
        && RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int)
        && (level as int) >= 2
        && RealmAt(old_s, rd).state == REALM_NEW
        && AddrIsGranuleAligned(old_s, vsmmu_ptr)
        && PaIsDelegable(old_s, vsmmu_ptr)
        && GranuleAt(old_s, vsmmu_ptr).state == VSMMU
        && AddrIsRttLevelAligned(old_s, ipa, level as int)
        && (ipa as int) >= (VsmmuAt(old_s, vsmmu_ptr).reg_base as int)
        && RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).level >= (level as int)
        && RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).rtte.state == UNASSIGNED
        && RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).rtte.ripas == EMPTY
        && (ipa as int) + (RttLevelSize(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).level) - 1)
                < (VsmmuAt(old_s, vsmmu_ptr).reg_top as int))
        ==> result.is_Ok()
            && RttEntryAt(new_s,
                    RttAt(new_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).rtt_addr),
                    RttEntryIndex(new_s, ipa, RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).level)).state == ASSIGNED_VSMMU
            && RttEntryAt(new_s,
                    RttAt(new_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).rtt_addr),
                    RttEntryIndex(new_s, ipa, RttWalk(old_s, RealmAt(old_s, rd), ipa, level as int, RMM_RTT_TREE_PRIMARY).level)).addr == vsmmu_ptr)
}