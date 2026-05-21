pub open spec fn rmi_vsmmu_map_spec(
    result: RmiCommandReturnCode,
    rd: Address,
    vsmmu_ptr: Address,
    ipa: Address,
    level: int,
    old_s: S,
    new_s: S,
) -> bool {
    let realm = RealmAt(old_s, rd);
    let vsmmu = VsmmuAt(old_s, vsmmu_ptr);
    let walk = RttWalk(old_s, realm, ipa, level, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);

    (!ImplFeatures(old_s).feat_da == FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
        && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(
        old_s,
        rd,
    ).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((!RttLevelIsValid(
        old_s,
        realm,
        level,
    ) || level < 2) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (realm.state != REALM_NEW
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsGranuleAligned(old_s, vsmmu_ptr)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(old_s, vsmmu_ptr)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(old_s, vsmmu_ptr).state != VSMMU
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsRttLevelAligned(old_s, ipa, level)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (UInt(ipa) < UInt(vsmmu.reg_base)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (walk.level < level ==> ResultEqual(
        result,
        RMI_ERROR_RTT,
    ) && result.payload == walk.level) && (walk.rtte.state != UNASSIGNED ==> ResultEqual(
        result,
        RMI_ERROR_RTT,
    ) && result.payload == walk.level) && (walk.rtte.ripas != EMPTY ==> ResultEqual(
        result,
        RMI_ERROR_RTT,
    ) && result.payload == walk.level) && ((UInt(ipa) + (RttLevelSize(old_s, walk.level) - 1)
        >= UInt(vsmmu.reg_top)) ==> ResultEqual(result, RMI_ERROR_RTT) && result.payload
        == walk.level) && ((ImplFeatures(old_s).feat_da == FEATURE_TRUE && AddrIsGranuleAligned(
        old_s,
        rd,
    ) && PaIsDelegable(old_s, rd) && GranuleAt(old_s, rd).state == RD && (RttLevelIsValid(
        old_s,
        realm,
        level,
    ) && level >= 2) && realm.state == REALM_NEW && AddrIsGranuleAligned(old_s, vsmmu_ptr)
        && PaIsDelegable(old_s, vsmmu_ptr) && GranuleAt(old_s, vsmmu_ptr).state == VSMMU
        && AddrIsRttLevelAligned(old_s, ipa, level) && UInt(ipa) >= UInt(vsmmu.reg_base)
        && walk.level >= level && walk.rtte.state == UNASSIGNED && walk.rtte.ripas == EMPTY && (
    UInt(ipa) + (RttLevelSize(old_s, walk.level) - 1) < UInt(vsmmu.reg_top))) ==> (result.is_Ok()
        && RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).state == ASSIGNED_VSMMU
        && RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).addr == vsmmu_ptr))
}