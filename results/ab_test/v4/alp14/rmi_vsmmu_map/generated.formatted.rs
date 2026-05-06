pub open spec fn RMI_VSMMU_MAP_spec(
    old_s: S,
    new_s: S,
    rd: Address,
    vsmmu_ptr: Address,
    ipa: Address,
    level: int,
    result: RmiCommandReturnCode,
) -> bool {
    let realm = RealmAt(old_s, rd);
    let vsmmu = VsmmuAt(old_s, vsmmu_ptr);
    let walk = RttWalk(old_s, realm, ipa, level, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let rtte_addr = RttAt(old_s, walk.rtt_addr);

    ((!ImplFeatures(old_s).feat_da) ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED)) && ((
    !AddrIsGranuleAligned(rd)) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((!PaIsDelegable(rd))
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((GranuleAt(old_s, rd).state != RD)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (((!RttLevelIsValid(old_s, realm, level)) || (
    level < 2)) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((realm.state != REALM_NEW)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((!AddrIsGranuleAligned(vsmmu_ptr))
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((!PaIsDelegable(vsmmu_ptr)) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && ((GranuleAt(old_s, vsmmu_ptr).state != VSMMU) ==> ResultEqual(result, RMI_ERROR_INPUT))
        && ((!AddrIsRttLevelAligned(ipa, level)) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (((
    ipa as int) < (vsmmu.reg_base as int)) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((walk.level
        < level) ==> ResultEqual(result, RMI_ERROR_RTT)) && ((walk.rtte.state != UNASSIGNED)
        ==> ResultEqual(result, RMI_ERROR_RTT)) && ((walk.rtte.ripas != EMPTY) ==> ResultEqual(
        result,
        RMI_ERROR_RTT,
    )) && ((((ipa as int) + (RttLevelSize(old_s, walk.level) - 1)) >= (vsmmu.reg_top as int))
        ==> ResultEqual(result, RMI_ERROR_RTT)) && (((ImplFeatures(old_s).feat_da) && (
    AddrIsGranuleAligned(rd)) && (PaIsDelegable(rd)) && (GranuleAt(old_s, rd).state == RD) && (
    RttLevelIsValid(old_s, realm, level)) && (level >= 2) && (realm.state == REALM_NEW) && (
    AddrIsGranuleAligned(vsmmu_ptr)) && (PaIsDelegable(vsmmu_ptr)) && (GranuleAt(
        old_s,
        vsmmu_ptr,
    ).state == VSMMU) && (AddrIsRttLevelAligned(ipa, level)) && ((ipa as int) >= (
    vsmmu.reg_base as int)) && (walk.level >= level) && (walk.rtte.state == UNASSIGNED) && (
    walk.rtte.ripas == EMPTY) && (((ipa as int) + (RttLevelSize(old_s, walk.level) - 1)) < (
    vsmmu.reg_top as int)) && (result.is_Ok())) ==> ((RttEntryAt(new_s, rtte_addr, entry_idx).state
        == ASSIGNED_VSMMU) && (RttEntryAt(new_s, rtte_addr, entry_idx).addr == vsmmu_ptr)))
}