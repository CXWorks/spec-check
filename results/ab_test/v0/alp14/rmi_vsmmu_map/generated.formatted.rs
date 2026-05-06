pub open spec fn RMI_VSMMU_MAP_spec(
    s: S,
    rd: Address,
    vsmmu_ptr: Address,
    ipa: Address,
    level: int,
    result: Result<(), RmiStatusCode>,
) -> bool {
    let realm = RealmAt(s, rd);
    let vsmmu = VsmmuAt(s, vsmmu_ptr);
    let walk = RttWalk(s, realm, ipa, level, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(s, ipa, walk.level);

    // da_supp: pre: ImplFeatures().feat_da != FEATURE_TRUE
    (ImplFeatures(s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
        &&
    // rd_align: pre: !AddrIsGranuleAligned(rd)
    (!AddrIsGranuleAligned(s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // rd_bound: pre: !PaIsDelegable(rd)
    (!PaIsDelegable(s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // rd_state: pre: GranuleAt(rd).state != RD
    (GranuleAt(s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // level_bound: pre: (!RttLevelIsValid(realm, level) || level < 2)
    ((!RttLevelIsValid(s, realm, level) || level < 2) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // realm_state: pre: realm.state != REALM_NEW
    (realm.state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // vsmmu_align: pre: !AddrIsGranuleAligned(vsmmu_ptr)
    (!AddrIsGranuleAligned(s, vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // vsmmu_bound: pre: !PaIsDelegable(vsmmu_ptr)
    (!PaIsDelegable(s, vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // vsmmu_state: pre: GranuleAt(vsmmu_ptr).state != VSMMU
    (GranuleAt(s, vsmmu_ptr).state != VSMMU ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // ipa_align: pre: !AddrIsRttLevelAligned(ipa, level)
    (!AddrIsRttLevelAligned(s, ipa, level) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // ipa_bound: pre: UInt(ipa) < UInt(vsmmu.reg_base)
    (UInt(ipa) < UInt(vsmmu.reg_base) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // rtt_walk: pre: walk.level < level
    (walk.level < level ==> ResultEqual(result, RMI_ERROR_RTT))
        &&
    // rtte_state: pre: walk.rtte.state != UNASSIGNED
    (walk.rtte.state != UNASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT))
        &&
    // rtte_ripas: pre: walk.rtte.ripas != EMPTY
    (walk.rtte.ripas != EMPTY ==> ResultEqual(result, RMI_ERROR_RTT))
        &&
    // rtte_bound: pre: (UInt(ipa) + (RttLevelSize(walk.level) - 1) >= UInt(vsmmu.reg_top))
    ((UInt(ipa) + (RttLevelSize(s, walk.level) - 1) >= UInt(vsmmu.reg_top)) ==> ResultEqual(
        result,
        RMI_ERROR_RTT,
    )) &&
    // Success conditions
    (result.is_Ok() ==> (walk.rtte.state == ASSIGNED_VSMMU && walk.rtte.addr == vsmmu_ptr))
}